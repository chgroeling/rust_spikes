use std::mem;

enum Node {
    Children(Vec<Node>),
    LeafNode(i32),
}

struct NodeIter<'a> {
    children: &'a [Node],
    parent: Option<Box<NodeIter<'a>>>,
}

impl NodeIter<'_> {
    fn new(node: &Node) -> NodeIter<'_> {
        NodeIter {
            children: std::slice::from_ref(node),
            parent: None,
        }
    }
}

impl Default for NodeIter<'_> {
    fn default() -> Self {
        NodeIter {
            children: &[],
            parent: None,
        }
    }
}

impl Iterator for NodeIter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // get the first element in the children slice
        let first = self.children.get(0);
        match first {
            Some(node) => match node {
                Node::LeafNode(leaf_node) => {
                    self.children = &self.children[1..];
                    return Some(*leaf_node);
                }

                Node::Children(children) => {
                    self.children = &self.children[1..];
                    *self = NodeIter {
                        children: children.as_slice(),
                        parent: Some(Box::new(mem::take(self))),
                    };

                    return self.next();
                }
            },

            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
        }
    }
}

fn dfs(node: Node) {
    match node {
        Node::Children(children) => {
            for i in children {
                dfs(i);
            }
        }
        Node::LeafNode(leaf) => {
            println!("LEAF: {}", leaf)
        }
    };
}

fn main() {
    #[rustfmt::skip]
    let node = Node::Children(vec![
        Node::LeafNode(1),
        Node::Children(vec![
          Node::LeafNode(2), 
          Node::Children(vec![
            Node::LeafNode(3)
          ]),
          Node::LeafNode(4), 
          Node::LeafNode(5)
        ]),
        Node::LeafNode(6)
    ]);


    let node_iter = NodeIter::new(&node);

    for i in node_iter {
        println!("LEAF {i}")
    }
}
