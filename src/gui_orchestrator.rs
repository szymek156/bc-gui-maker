use std::{
    fs::File,
    io::Write,
    iter::FromIterator,
    path::{Path, PathBuf},
};

use regex::Regex;

#[derive(Debug, Default, Clone, Copy)]
pub struct Dimension {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Default)]
struct Text {
    dim: Dimension,
    name: &'static str,
    format: Option<&'static str>,
    // Font may be set by the user, then
    // orchestrator is not allowed to change it
    // TODO: Change to enum
    font_size: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Tile {
    dim: Dimension,
    text: Text,
}

#[derive(Debug, Default)]
pub struct List {
    // Dimension of whole element
    dim: Dimension,
    // Elements which user is able to select
    elements: Vec<Tile>,
    // If visible_elements is less than # of elements in total
    // scroll the view
    visible_elements: usize,
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

fn render_60fps_rectangle(tile: &Tile) -> String {
    format!(
        r#"Rectangle {{
    x: {x}phx;
    y: {y}phx;
    width: {width}phx;
    height: {height}phx;
    background: whitesmoke;
    border-color: black;
    border-width: 0px;
    Text {{
        //x: {{x_text}}phx;
        y: {y_text}phx;
        width: 100%;
        height: 100%;
        text: "{name}";
        font-size: {font_size}phx;
        // That's the closest font to the one on BC display,
        // still very different
        font-family: "noto mono";
        // vertical-alignment: center;
        horizontal-alignment: center;
    }}
}}
"#,
        x = tile.dim.x,
        y = tile.dim.y,
        width = tile.dim.width,
        height = tile.dim.height,
        // Use horizontal-alignment, since fonts differ significantly
        // between 60fps and BC display
        // x_text = tile.text.dim.x,
        y_text = tile.text.dim.y,
        name = tile.text.name,
        font_size = tile.text.font_size.unwrap()
    )
}
/// Returns a tuple (Dynamic, Static) widgets
fn render_60fps_widgets(root: &Node) -> (String, String) {
    match root {
        Node::V(nodes) | Node::H(nodes) => nodes
            .iter()
            .map(|node| render_60fps_widgets(node))
            .fold((String::default(), String::default()), |mut acc, x| {
                acc.0 += &x.0;
                acc.1 += &x.1;

                acc
            }),
        Node::HH(split) => {
            let (l_dyn, l_stat) = render_60fps_widgets(&split.first);
            let (r_dyn, r_stat) = render_60fps_widgets(&split.second);

            // TODO: format! ??
            (l_dyn + &r_dyn, l_stat + &r_stat)
        }

        Node::Tile(tile) => (render_60fps_rectangle(tile), String::default()),
        Node::HorizontalLine(dim) | Node::VerticalLine(dim) => (
            String::default(),
            format!(
                r#"Rectangle {{
            x: {x}phx;
            y: {y}phx;
            width: {width}phx;
            height: {height}phx;
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
        Node::VListWidget(list) => (
            list.elements.iter().fold(String::new(), |acc, tile| {
                acc + &render_60fps_rectangle(tile)
            }),
            String::default(),
        ),
    }
}

/// Gets gui layout and creates a sixty fps markup String representing that layout.
pub fn render_to_60fps(root: &Node, d: &Dimension) -> String {
    let (tiles, static_elements) = render_60fps_widgets(&root);

    let result = format!(
        "MainWindow := Window{{
        width: {width}phx;
        height: {height}phx;
        background: red;

        {tiles}

        {static_elements}
    }}
    ",
        width = d.width,
        height = d.height,
        tiles = tiles,
        static_elements = static_elements
    );

    // println!("Rendered:\n {}", result);

    write_to_file(&result, "ui/main.60");
    result
}

fn render_bc_widgets(root: &Node) -> (String, String) {
    match root {
        Node::V(nodes) | Node::H(nodes) => nodes.iter().map(|node| render_bc_widgets(node)).fold(
            (String::default(), String::default()),
            |mut acc, x| {
                acc.0 += &x.0;
                acc.1 += &x.1;

                acc
            },
        ),
        Node::HH(split) => {
            let (l_dyn, l_stat) = render_bc_widgets(&split.first);
            let (r_dyn, r_stat) = render_bc_widgets(&split.second);

            // TODO: format! ??
            (l_dyn + &r_dyn, l_stat + &r_stat)
        }
        Node::Tile(tile) => {
            let format_msg = match tile.text.format {
                Some(format) => format!(
                    r#"snprintf(message, msg_size, "{format}", data.);"#,
                    format = format
                ),
                None => format!(
                    r#"snprintf(message, msg_size, "{text}");"#,
                    text = tile.text.name
                ),
            };

            (
                format!(
                    r#"// {name}
display_->enqueueDraw(
    [&](Paint &paint) {{
        const int msg_size = 128;
        char message[msg_size];

    {format_msg}
    paint.DrawStringAt({x}, {y}, message, &Font{font}, COLORED);

}},
{{{x0}, {y0}, {x1}, {y1}}});

"#,
                    name = tile.text.name,
                    format_msg = format_msg,
                    x = tile.dim.x + tile.text.dim.x,
                    y = tile.dim.y + tile.text.dim.y,
                    font = tile.text.font_size.unwrap(),
                    // Shrink refresh area so static elements will not be wiped out
                    x0 = tile.dim.x + 1,
                    y0 = tile.dim.y + 1,
                    x1 = tile.dim.x + tile.dim.width - 1,
                    y1 = tile.dim.y + tile.dim.height - 1
                ),
                String::default(),
            )
        }
        Node::HorizontalLine(dim) => (
            String::default(),
            format!(
                r#"    paint.DrawHorizontalLine({x}, {y}, {line_width}, COLORED);
                "#,
                x = dim.x,
                y = dim.y,
                line_width = dim.width
            ),
        ),
        Node::VerticalLine(dim) => (
            String::default(),
            format!(
                r#"    paint.DrawVerticalLine({x}, {y}, {line_height}, COLORED);
                "#,
                x = dim.x,
                y = dim.y,
                line_height = dim.height
            ),
        ),
        Node::VListWidget(_) => {
            // TODO: implement later
            (String::default(), String::default())
        }
    }
}

/// Gets raw font size and samples it to sizes supported by BC display
fn set_bc_font_size(tile: &mut Tile) {
    let char_len = tile.text.name.chars().count();

    tile.text.font_size = Some(8);
    // Try to fit biggest font in the Rectangle
    for font in [24, 20, 16, 12, 8] {
        let char_width = get_bc_font_width(font);
        let str_width = char_width * char_len;

        if str_width < tile.dim.width && font < tile.dim.height {
            tile.text.font_size = Some(font);
            break;
        }
    }
}

/// Width in pixels for single character depending on the font size.
/// Taken from the font source code.
fn get_bc_font_width(font_size: usize) -> usize {
    match font_size {
        8 => 5,
        12 => 7,
        16 => 11,
        20 => 14,
        24 => 17,
        _ => unreachable!("got font_size {}", font_size),
    }
}

fn center_text(tile: &mut Tile) {
    let font_size = tile.text.font_size.unwrap();
    let char_width = get_bc_font_width(font_size);
    let char_len = tile.text.name.chars().count();

    // If str goes beyond the Tile, clamp it's width
    let str_width = (char_width * char_len).min(tile.dim.width);

    // Text x, y relative to parent Tile (tile.dim + tile.text.dim)
    tile.text.dim.x = (tile.dim.width - str_width) / 2;
    // Assuming here font size describes amount of pixels,
    // subtract from tile.dim.height to get amount of free space
    // and div by 2 to have it vertically centered

    tile.text.dim.y = (tile.dim.height - font_size) / 2;
}

fn render_to_bc(root: &Node, d: &Dimension) -> String {
    let (tiles, static_elements) = render_bc_widgets(&root);

    let result = format!(
        "
    // Following code is generated automagically,
    // don't bother understand it.

    {tiles}

    void StatusView::drawStatic() {{
        display_->enqueueStaticDraw(
            [](Paint &paint) {{
            {static_elements}
            }},
            // Rectangle needs to cover whole widget area
            {{0, 0, display_->getWidth(), 13}});
    }}
    ",
        tiles = tiles,
        static_elements = static_elements
    );

    println!("Rendered:\n {}", result);

    result
}

fn write_to_file(gui: &String, path: &str) {
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
    fn v_layout_with_h_layouts_splits_them_via_v_lines() {
        // V:
        //   H: A | B
        //   H: C | D
        let mut gui = v_layout([
            v_line(), // splits A | B
            h_layout([tile("A"), tile("B")]),
            v_line(), // splits C | D
            h_layout([tile("C"), tile("D")]),
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
    fn h_layout_with_v_layouts_splits_them_via_h_lines() {
        // H: V: A  V: C
        //       --    --
        //       B ,   D
        let mut gui = h_layout([
            h_line(),
            // splits
            // A
            // --
            // B
            v_layout([tile("A"), tile("B")]),
            h_line(),
            // splits
            // C
            // --
            // D
            v_layout([tile("C"), tile("D")]),
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
    fn bc_test_page() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([
                    tile("02/09/21").with_format("%d/%m/%y"),
                    tile("19:34:20").with_format("%T"),
                ]),
                v_layout([
                    tile("in view / tracked"),
                    tile("13 / 11").with_format("%d / %d").with_font_size(12),
                ]),
            ]),
            // Current implementation of invalidate_dimensions
            // makes {h,v}_line split tiles defined after them
            // so they 'stick' to the bottom
            h_line(),
            v_line(),
            h_layout([
                v_layout([
                    tile("23.19[*C]").with_format("%5.2f[*C]"),
                    tile("133.94[m]").with_format("%5.2f[m]"),
                ]),
                v_layout([
                    tile("Hit button below"),
                    tile("to calculate your"),
                    tile("BMI"),
                ]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn welcome() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            h_layout([v_layout([
                tile("21:37:07").with_format("%T"),
                h_line(),
                tile("02/09/21").with_format("%d/%m/%y"),
            ])]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn select_activity() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([tile("Activity"), h_line(), tile("")]),
                v_list([
                    "Running",
                    "Cycling",
                    "Hiking",
                    "Ind. Cycling",
                    "Yoga",
                    "Swimming",
                ])
                .with_font_size(16),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn select_running_workouts() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([tile("Running"), h_line(), tile("Workouts")]),
                v_layout([
                    tile("5k").with_font_size(16),
                    tile("10k").with_font_size(16),
                    tile("Half Marathon").with_font_size(16),
                    tile("Marathon").with_font_size(16),
                    tile("Cooper Test").with_font_size(16),
                ]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn activity_running_cooper_test() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([tile("Running"), h_line(), tile("Cooper Test")]),
                v_layout([tile("Do It"), tile("View")]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn activity_running_cooper_test_view() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            h_split(
                v_layout([tile("Cooper Test"), h_line()]),
                0.2,
                v_layout([
                    tile("Step 1: Warmup").with_font_size(12),
                    tile("Step 2: Run for your life for 12 mins").with_font_size(12),
                    tile("Step 3: Note the distance").with_font_size(12),
                    tile("Step 4: Look at the table").with_font_size(12),
                    // TODO: all steps disappear after adding another element
                    // Because for .60fps font of size 16 does not fit in a
                    // rect of height 18 and clips the text
                    // tile("Step 5"),
                ]),
            ),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }
}
