use crate::common::{self, Dimension, Node, Tile};

fn render_60fps_rectangle(tile: &Tile) -> String {
    format!(
        r#"Rectangle {{
    x: {x}phx;
    y: {y}phx;
    width: {width}phx;
    height: {height}phx;
    background: silver;
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
        font-family: "Ubuntu Mono";
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

    common::write_to_file(&result, "ui/main.60");
    result
}