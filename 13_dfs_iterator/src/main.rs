//! Interesting example of a tree datastructure which is traversed by an iterator.
//!
//! This example shows a lot of interesting concepts around memory management in rust.

use std::mem;

enum Node {
    Children(Vec<Node>),
    LeafNode(i32),
}

impl Node {
    // Create an iterator
    fn iter<'a>(&'a self) -> NodeIter<'a> {
        NodeIter::new(self)
    }
}

struct NodeIter<'a> {
    children: &'a [Node], // a slice of Nodes

    // The Box type allocates NodeIter on the Heap. This is needed because
    // a struct cannot contain itself. This would lead to an infinite stack consumption
    parent: Option<Box<NodeIter<'a>>>,
}

impl<'a> NodeIter<'a> {
    /// Returns a new node iterator
    fn new<'b>(node: &'b Node) -> NodeIter<'b> {
        // No self is used. This is therefore an associated function
        NodeIter {
            children: std::slice::from_ref(node),
            parent: None,
        }
    }
}

// The Default trait gives the possibility to return a default value of a type
impl<'a> Default for NodeIter<'a> {
    fn default() -> Self {
        NodeIter {
            children: &[],
            parent: None,
        }
    }
}

// The Iterator trait gives access to all iterator specific functions.
impl<'a> Iterator for NodeIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // get the first element in the children slice or None
        let first = self.children.get(0);

        match first {
            Some(node) => match node {
                Node::LeafNode(leaf_node) => {
                    // reduce the slice
                    self.children = &self.children[1..];
                    Some(*leaf_node)
                }

                Node::Children(children) => {
                    self.children = &self.children[1..];

                    // the mutable borrowed self is replaced by mem::take with the default value
                    // given by the default trait
                    let new_parent = Some(Box::new(mem::take(self)));

                    // Create a new iterator which should be the new "self"
                    let new_self = NodeIter {
                        children: children.as_slice(),
                        parent: new_parent,
                    };

                    *self = new_self; // a new self

                    self.next()
                }
            },

            // take what is behind self.parent and leave None instead
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

    println!("First iteration");
    for i in node.iter() {
        println!("item: {i}")
    }

    println!("\nSecond iteration");
    for i in node.iter() {
        println!("item:{i}")
    }

}
