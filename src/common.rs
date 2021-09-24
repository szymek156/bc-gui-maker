use std::{
    fs::File,
    io::Write,
    iter::FromIterator,
    path::{Path, PathBuf},
};

use crate::bc_render::{center_text, set_bc_font_size};

pub fn write_to_file(gui: &String, path: &str) {
    let full_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), path]);
    let path = Path::new(&full_path);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {:?}: {}", path, why),
        Ok(file) => file,
    };

    match file.write_all(gui.as_bytes()) {
        Err(why) => panic!("couldn't write to {:?}: {}", path, why),
        Ok(_) => println!("successfully wrote to {:?}", path),
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Dimension {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Default)]
pub struct Text {
    pub dim: Dimension,
    pub name: &'static str,
    pub format: Option<&'static str>,
    // Font may be set by the user, then
    // orchestrator is not allowed to change it
    // TODO: Change to enum
    pub font_size: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Tile {
    pub dim: Dimension,
    pub text: Text,
}

#[derive(Debug, Default)]
pub struct List {
    // Dimension of whole element
    pub dim: Dimension,
    // Elements which user is able to select
    pub elements: Vec<Tile>,
    // If visible_elements is less than # of elements in total
    // scroll the view
    pub visible_elements: usize,
}
#[derive(Debug)]
pub struct HardSplit {
    pub first: Box<Node>,
    pub first_occupation_percent: f64,
    pub second: Box<Node>,
    pub second_occupation_percent: f64,
}

#[derive(Debug)]
pub enum Node {
    /// Vertical layout, splits area evenly among elements in vector
    V(Vec<Box<Node>>),

    /// Horizontal layout, splits area evenly among elements in vector
    H(Vec<Box<Node>>),

    // Hard Horizontal split, splits into two parts, uneven
    HH(HardSplit),

    Tile(Tile),
    HorizontalLine(Dimension),
    VerticalLine(Dimension),
    VListWidget(List),
}
/// [] [] []
pub fn h_layout<T>(elements: T) -> Node
where
    T: IntoIterator<Item = Node>,
{
    Node::H(elements.into_iter().map(|e| Box::new(e)).collect())
}

/// []
/// []
/// []
/// T, where T : IntoIterator<Item = Node>
/// Meaning any kind of collection, which implements IntoIterator,
/// where returning value is Node.
/// That includes arrays - of any size!
pub fn v_layout<T>(elements: T) -> Node
where
    T: IntoIterator<Item = Node>,
{
    // Here real magic begins, because possibly stack allocated element
    // becomes heap allocated
    Node::V(elements.into_iter().map(|e| Box::new(e)).collect())
}

pub fn h_split(left: Node, left_percent: f64, right: Node) -> Node {
    Node::HH(HardSplit {
        first: Box::new(left),
        first_occupation_percent: left_percent,
        second: Box::new(right),
        second_occupation_percent: 1.0 - left_percent,
    })
}

pub fn h_line() -> Node {
    Node::HorizontalLine(Dimension::default())
}

pub fn v_line() -> Node {
    Node::VerticalLine(Dimension::default())
}

pub fn tile(name: &'static str) -> Node {
    Node::Tile(Tile {
        text: Text {
            name,
            ..Default::default()
        },
        // Some type magic inference?
        ..Default::default()
    })
}

pub fn v_list<T>(elements: T) -> Node
where
    T: IntoIterator<Item = &'static str>,
{
    let elements: Vec<_> = elements
        .into_iter()
        .map(|el| Tile {
            text: Text {
                name: el,
                ..Default::default()
            },
            ..Default::default()
        })
        .collect();

    Node::VListWidget(List {
        visible_elements: elements.len(),
        elements,
        ..Default::default()
    })
}

impl Node {
    pub fn with_format(mut self, format: &'static str) -> Self {
        // TODO: any better way to do this?
        match self {
            Node::Tile(ref mut tile) => {
                tile.text.format = Some(format);
                self
            }
            _ => panic!("Cannot set format on {:?}", self),
        }
    }

    /// Sets explicitly font size. Make sure it will fit in the
    /// Rectangle height, otherwise dim validation will panic
    pub fn with_font_size(mut self, size: usize) -> Self {
        match self {
            Node::Tile(ref mut tile) => {
                tile.text.font_size = Some(size);
                self
            }
            Node::VListWidget(ref mut list) => {
                let _: Vec<_> = list
                    .elements
                    .iter_mut()
                    .map(|element| element.text.font_size = Some(size))
                    .collect();
                self
            }
            _ => panic!("Cannot set font_size on {:?}", self),
        }
    }
}

/// Gets root of the gui, and updates leaf dimensions with
/// correct x, y, width, height values
pub fn invalidate_dimensions(root: &mut Node, d: &Dimension) {
    match root {
        Node::V(nodes) => {
            // Get number of elements which are NOT {h,v}_lines
            let len = nodes
                .iter()
                .filter(|e| match e.as_ref() {
                    Node::HorizontalLine(_) | Node::VerticalLine(_) => false,
                    _ => true,
                })
                .count();

            // Split area evenly by nodes which are NOT {h,v}_lines
            let height = ((d.height / len) as f64).round() as usize;

            let mut h_lines_count = 0;

            let _: Vec<_> = nodes
                .iter_mut()
                .enumerate()
                .map(|(idx, node)| {
                    // H or V lines overlap on other widgets,
                    // so each time h_line is present we substract
                    // the idx, so widget goes to the place under
                    // the h_line
                    // Without correction:
                    // [widget1]
                    // [h_line]
                    // [empty_space]
                    // [widget2]
                    // [widget3] - outside the screen!

                    // With correction:
                    // [widget1]
                    // [h_line]
                    // [widget2]
                    // [widget3] - bottom of the screen

                    let idx = idx - h_lines_count;

                    // In vertical layout x coord for v_lines must be corrected
                    let mut x = d.x;
                    match node.as_ref() {
                        Node::HorizontalLine(_) => {
                            h_lines_count += 1;
                        }
                        Node::VerticalLine(_) => {
                            h_lines_count += 1;
                            x = d.x + d.width / 2;
                        }
                        _ => (),
                    };

                    invalidate_dimensions(
                        node,
                        &Dimension {
                            y: d.y + idx * height,
                            height,
                            x: x,
                            ..*d
                        },
                    )
                })
                .collect();
        }
        Node::H(nodes) => {
            // Get number of elements which are NOT {h,v}_lines
            let len = nodes
                .iter()
                .filter(|e| match e.as_ref() {
                    Node::HorizontalLine(_) | Node::VerticalLine(_) => false,
                    _ => true,
                })
                .count();

            // Split area evenly by nodes which are NOT {h,v}_lines
            let width = ((d.width / len) as f64).round() as usize;

            let mut h_lines_count = 0;

            let _: Vec<_> = nodes
                .iter_mut()
                .enumerate()
                .map(|(idx, node)| {
                    let idx = idx - h_lines_count;

                    let mut y = d.y;
                    match node.as_ref() {
                        Node::HorizontalLine(_) => {
                            h_lines_count += 1;
                            y = d.y + d.height / 2;
                        }
                        Node::VerticalLine(_) => {
                            h_lines_count += 1;
                        }
                        _ => (),
                    };

                    invalidate_dimensions(
                        node,
                        &Dimension {
                            x: d.x + idx * width,
                            width,
                            y: y,
                            ..*d
                        },
                    )
                })
                .collect();
        }
        Node::HH(split) => {
            let up_height = (d.height as f64 * split.first_occupation_percent) as usize;
            let down_height = d.height - up_height;
            invalidate_dimensions(
                &mut split.first,
                &Dimension {
                    x: d.x,
                    y: d.y,
                    width: d.width,
                    height: up_height,
                },
            );
            invalidate_dimensions(
                &mut split.second,
                &Dimension {
                    x: d.x,
                    y: d.y + up_height,
                    width: d.width,
                    height: down_height,
                },
            );
        }
        Node::Tile(tile) => {
            tile.dim = *d;

            // let font_size = (tile.dim.width.min(tile.dim.height) as f64 * 0.75) as usize;
            if tile.text.font_size.is_none() {
                set_bc_font_size(tile);
            }

            center_text(tile);
        }
        Node::HorizontalLine(dim) => {
            const MARGIN: usize = 13;
            dim.x = d.x + MARGIN;
            dim.y = d.y;
            dim.width = d.width - 2 * MARGIN;
            dim.height = 1;
        }
        Node::VerticalLine(dim) => {
            const MARGIN: usize = 3;
            dim.x = d.x;
            dim.y = d.y + MARGIN;
            dim.width = 1;
            dim.height = d.height - 2 * MARGIN;
        }
        Node::VListWidget(list) => {
            list.dim = *d;

            const MARGIN: usize = 1;

            let tile_height = d.height / list.visible_elements;

            for (idx, tile) in &mut list.elements.iter_mut().enumerate() {
                let offset = idx * tile_height;

                tile.dim = Dimension {
                    x: list.dim.x + MARGIN,
                    y: d.y + offset,
                    width: list.dim.width - 2 * MARGIN,
                    height: tile_height,
                };

                center_text(tile)
            }
        }
    }
}
