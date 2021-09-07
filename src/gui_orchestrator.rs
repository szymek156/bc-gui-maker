#[derive(Debug, Default)]
pub struct Tile {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    name: &'static str,
}

#[derive(Debug)]
pub enum Node {
    Int(Vec<Box<Node>>),
    Leaf(Tile),
}
/// [] [] []
pub fn h_layout(elements: [Node]) -> Node {
    match elements {
        Node::Int(laouts) => Node::Int(laouts),
        Node::Leaf(leaf) => Node::Leaf(leaf),
    }
}

/// []
/// []
/// []
pub fn v_layout(elements: [Node]) -> Node {
    match elements {
        Node::Int(laouts) => Node::Int(laouts),
        Node::Leaf(leaf) => Node::Leaf(leaf),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_gui() {
        let gui = v_layout([
            Box::new(Node::Leaf(Tile {
                name: "text",
                ..Tile::default()
            })),
        ]);

        let gui = Node::Int(vec![
            // vert layout
            Box::new(Node::Int(vec![
                // h layout
                Box::new(Node::Int(vec![
                    // v layout
                    Box::new(Node::Leaf(Tile {
                        name: "text",
                        ..Tile::default()
                    })),
                    Box::new(Node::Leaf(Tile {
                        name: "%d",
                        ..Tile::default()
                    })),
                    Box::new(Node::Int(vec![
                        // v layout
                        Box::new(Node::Leaf(Tile {
                            name: "text",
                            ..Tile::default()
                        })),
                        Box::new(Node::Leaf(Tile {
                            name: "%d",
                            ..Tile::default()
                        })),
                    ])),
                ])),
            ])),
            Box::new(Node::Int(vec![ // h layout

            ])),
        ]);

        println!("gui: {:#?}", gui);
    }
}
