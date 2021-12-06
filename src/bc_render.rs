use std::os::linux::raw;

use crate::common::{Dimension, Node, Tile};

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
        Node::VListWidget(list) => {
            let raw_elements: Vec<_> = list.elements.iter().map(|tile| tile.text.name).collect();
            // That magic in .replace is ... magic
            let raw_elements = format!("{:?}", raw_elements).replace(&['[', ']'][..], "");
            (
                format!(
                    r#"
// CTOR
VListWidget(display, {{{activities}}}, Font{font}, {{{x0}, {y0}, {x1}, {y1}}})


"#,
                    activities = raw_elements,
                    font = list.font_size.unwrap(),
                    x0 = list.dim.x + 1,
                    y0 = list.dim.y + 1,
                    x1 = list.dim.x + list.dim.width - 1,
                    y1 = list.dim.y + list.dim.height - 1
                ),
                String::default(),
            )
        }
    }
}

/// Gets raw font size and samples it to sizes supported by BC display
pub fn set_bc_font_size(tile: &mut Tile) {
    let char_len = tile.text.name.chars().count();

    tile.text.font_size = Some(8);
    // Try to fit biggest font in the Rectangle
    for font in [56, 42, 31, 24, 19] {
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
        19 => 11,
        24 => 14,
        31 => 18,
        42 => 24,
        56 => 32,
        _ => unreachable!("got font_size {}", font_size),
    }
}

/// Center in vertical na horizontal dimensions of the Tile
pub fn center_text(tile: &mut Tile) {
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

/// Generates C++ code so you don't need to type it anymore!
pub fn render_to_bc(root: &Node, d: &Dimension) -> String {
    let (tiles, static_elements) = render_bc_widgets(&root);

    let result = format!(
        "
    // Following code is generated automagically,
    // don't bother understand it.

    {tiles}

    void StatusView::drawStatic() {{
        display_->enqueueStaticDraw(
            [&](Paint &paint) {{
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
