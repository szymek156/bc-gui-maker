use std::{
    fs::File,
    io::Write,
    iter::FromIterator,
    os::linux::raw::stat,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Dimension {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Default)]
pub struct Tile {
    dim: Dimension,
    name: &'static str,
}

#[derive(Debug)]
pub struct HardSplit {
    first: Box<Node>,
    first_occupation_percent: f64,
    second: Box<Node>,
    second_occupation_percent: f64,
}

#[derive(Debug)]
pub enum Node {
    /// Vertical layout, splits area evenly among elements in vector
    V(Vec<Box<Node>>),

    /// Horizontal layout, splits area evenly among elements in vector
    H(Vec<Box<Node>>),

    // Hard Horizontal split, splits into two parts, uneven
    HH(HardSplit),

    Leaf(Tile),
    HorizontalLine(Dimension),
    VerticalLine(Dimension),
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
    Node::Leaf(Tile {
        name,
        // Some type magic inference?
        ..Default::default()
    })
}

/// Gets root of the gui, and updates leaf dimensions with
/// correct x, y, width, height values
pub fn invalidate_dimensions(root: &mut Node, d: &Dimension) {
    match root {
        Node::V(nodes) => {
            // Get number of elements which are NOT h_lines
            let len = nodes
                .iter()
                .filter(|e| match e.as_ref() {
                    Node::HorizontalLine(_) => false,
                    _ => true,
                })
                .count();

            // Split area evenly by nodes which are NOT h_lines
            let height = d.height / len;

            let mut h_lines_count = 0;

            let _: Vec<_> = nodes
                .iter_mut()
                .enumerate()
                .map(|(idx, node)| {
                    // H lines overlap on other widgets,
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

                    match node.as_ref() {
                        Node::HorizontalLine(_) => {
                            h_lines_count += 1;
                        }
                        _ => (),
                    };

                    invalidate_dimensions(
                        node,
                        &Dimension {
                            y: d.y + idx * height,
                            height,
                            ..*d
                        },
                    )
                })
                .collect();
        }
        Node::H(nodes) => {
            let len = nodes.len();
            let width = d.width / len;
            let _: Vec<_> = nodes
                .iter_mut()
                .enumerate()
                .map(|(idx, node)| {
                    invalidate_dimensions(
                        node,
                        &Dimension {
                            x: d.x + idx * width,
                            width,
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
        Node::Leaf(tile) => {
            tile.dim = *d;
        }
        Node::HorizontalLine(dim) => {
            const MARGIN: usize = 13;
            dim.x = d.x + MARGIN;
            dim.y = d.y;
            dim.width = d.width - 2 * MARGIN;
            dim.height = 1;
        }
        Node::VerticalLine(dim) => todo!(),
    }
}

/// Returns a tuple (Dynamic, Static) widgets
fn render_widgets(root: &Node) -> (String, String) {
    match root {
        Node::V(nodes) | Node::H(nodes) => nodes.iter().map(|node| render_widgets(node)).fold(
            (String::default(), String::default()),
            |mut acc, x| {
                acc.0 += &x.0;
                acc.1 += &x.1;

                acc
            },
        ),
        Node::HH(split) => {
            let (l_dyn, l_stat) = render_widgets(&split.first);
            let (r_dyn, r_stat) = render_widgets(&split.second);

            // TODO: format! ??
            (l_dyn + &r_dyn, l_stat + &r_stat)
        }

        Node::Leaf(tile) => {
            let font_size = (tile.dim.width.min(tile.dim.height) as f64 * 0.75) as usize;
            (
                format!(
                    r#"Rectangle {{
            x: {x}px;
            y: {y}px; 
            width: {width}px;
            height: {height}px; 
            background: blue;
            border-color: black;
            border-width: 0px;
            Text {{
                width: 100%;
                height: 100%; 
                text: "{name}"; 
                font-size: {font_size}px; 
                vertical-alignment: center; 
                horizontal-alignment: center;                     
            }}
        }}
        "#,
                    x = tile.dim.x,
                    y = tile.dim.y,
                    width = tile.dim.width,
                    height = tile.dim.height,
                    name = tile.name,
                    font_size = font_size
                ),
                String::default(),
            )
        }
        Node::HorizontalLine(dim) => (
            String::default(),
            format!(
                r#"Rectangle {{
            x: {x}px;
            y: {y}px; 
            width: {width}px;
            height: {height}px; 
            background: black;
            border-color: black;
            border-width: 0px;
        }}
        "#,
                x = dim.x,
                y = dim.y,
                width = dim.width,
                height = dim.height,
            ),
        ),
        Node::VerticalLine(_) => todo!(),
    }
}

/// Gets gui layout and creates a sixty fps markup String representing that layout.
pub fn render_to_60fps(root: &Node, d: &Dimension) -> String {
    let (tiles, static_elements) = render_widgets(&root);

    let result = format!(
        "MainWindow := Window{{
        width: {width}phx;
        height: {height}phx;
        background: green;

        {tiles}

        {static_elements}
    }}
    ",
        width = d.width,
        height = d.height,
        tiles = tiles,
        static_elements = static_elements
    );

    println!("Rendered:\n {}", result);

    write_to_file(&result, "ui/main.60");
    result
}

fn write_to_file(gui: &String, path: &str) {
    let full_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), path]);
    let path = Path::new(&full_path);

    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(gui.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_gui() {
        let mut gui = v_layout([
            h_layout([
                h_layout([tile("A"), tile("B")]),
                v_layout([tile("C"), tile("D")]),
            ]),
            h_layout([
                tile("E"),
                h_layout([tile("F"), v_layout([tile("G"), tile("H")])]),
            ]),
        ]);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);
    }

    #[test]
    fn bc_welcome() {
        let status_bar = h_layout([tile("status bar")]);
        let welcome_page = v_layout([
            h_line(),
            h_layout([
                v_layout([tile("02/09/21"), tile("19:34:20")]),
                v_layout([tile("in view/tracked"), tile("15/6")]),
            ]),
            // v_line(),
            h_line(),
            h_layout([
                v_layout([tile("23.19[*C]"), tile("33.94[m]")]),
                v_layout([
                    tile("Hit button below"),
                    tile("to calculate your"),
                    tile("BMI"),
                ]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.15, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);
    }
}
