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
pub fn h_layout<T>(elements: T) -> Node where T : IntoIterator<Item = Node> {
    Node::Int(elements.into_iter().map(|e| Box::new(e)).collect())
}

/// []
/// []
/// []
/// T, where T : IntoIterator<Item = Node>
/// Meaning any kind of collection, which implements IntoIterator, where returning
/// value is Node.
/// That includes arrays - of any size!
pub fn v_layout<T>(elements: T) -> Node where T : IntoIterator<Item = Node> {
    // Here real magic begins, because possibly stack allocated element
    // becomes heap allocated
    Node::Int(elements.into_iter().map(|e| Box::new(e)).collect())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_gui() {
        let gui = v_layout([
            Node::Leaf(Tile {
                name: "text",
                // Some type magic inference?
                ..Default::default()
            }),
            Node::Leaf(Tile {
                name: "text",
                // Some type magic inference?
                ..Default::default()
            }),
        ]);

        // let gui = Node::Int(vec![
        //     // vert layout
        //     Box::new(Node::Int(vec![
        //         // h layout
        //         Box::new(Node::Int(vec![
        //             // v layout
        //             Box::new(Node::Leaf(Tile {
        //                 name: "text",
        //                 ..Tile::default()
        //             })),
        //             Box::new(Node::Leaf(Tile {
        //                 name: "%d",
        //                 ..Tile::default()
        //             })),
        //             Box::new(Node::Int(vec![
        //                 // v layout
        //                 Box::new(Node::Leaf(Tile {
        //                     name: "text",
        //                     ..Tile::default()
        //                 })),
        //                 Box::new(Node::Leaf(Tile {
        //                     name: "%d",
        //                     ..Tile::default()
        //                 })),
        //             ])),
        //         ])),
        //     ])),
        //     Box::new(Node::Int(vec![ // h layout

        //     ])),
        // ]);

        println!("gui: {:#?}", gui);
    }
}
