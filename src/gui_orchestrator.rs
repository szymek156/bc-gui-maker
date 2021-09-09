#[derive(Debug, Default)]
pub struct Tile {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    name: &'static str,
}

pub struct Dimension {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum Node {
    V(Vec<Box<Node>>),
    H(Vec<Box<Node>>),
    Leaf(Tile),
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
            let len = nodes.len();
            // Divide screen height area into equal parts
            let height = d.height / len;

            let _: Vec<_> = nodes
                .iter_mut()
                .enumerate()
                .map(|(idx, node)| {
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
        Node::Leaf(tile) => {
            tile.x = d.x;
            tile.y = d.y;
            tile.width = d.width;
            tile.height = d.height;
        }
    }
}

fn render_tiles(root: &Node) -> String {
    match root {
        // String is also an collection, so you can collect() to it, type is deduced automatically
        // This is soo f*kin elegant! No accumulator needed!
        Node::V(nodes) | Node::H(nodes) => nodes.iter().map(|node| render_tiles(node)).collect(),
        Node::Leaf(tile) => {
            let font_size = (tile.width.min(tile.height) as f64 * 0.85) as usize;
            format!(
                r#"Rectangle {{
                x: {x}px;
                y: {y}px; 
                width: {width}px;
                height: {height}px; 
                background: blue;
                border-color: black;
                border-width: 1px;
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
                x = tile.x,
                y = tile.y,
                width = tile.width,
                height = tile.height,
                name = tile.name,
                font_size=font_size
            )
        }
    }
}

/// Gets gui layout and creates a sixty fps markup String representing that layout.
pub fn render_to_60fps(root: &Node, d: &Dimension) -> String{
    let tiles = render_tiles(&root);

    let result = format!(
        "MainWindow := Window{{
        width: {width}phx;
        height: {height}phx;

        {tiles}
    }}
    ",
        width = d.width,
        height = d.height,
        tiles = tiles
    );

    println!("Rendered:\n {}", result);

    result
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
}
