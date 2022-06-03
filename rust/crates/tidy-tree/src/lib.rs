#![allow(dead_code, unused_imports, unused_variables)]
pub mod geometry;
mod layout;
mod node;
use std::{any::Any, collections::HashMap, ptr::NonNull};

use layout::BoundingBox;
pub use layout::{BasicLayout, Layout};
pub use node::Node;

pub struct TidyTree {
    root: Node,
    layout: Box<dyn Layout>,
    map: HashMap<usize, NonNull<Node>>,
}

impl TidyTree {
    pub fn new() -> Self {
        TidyTree {
            root: Default::default(),
            layout: Box::new(BasicLayout {
                parent_child_margin: 10.,
                peer_margin: 10.,
            }),
            map: HashMap::new(),
        }
    }

    pub fn layout(&mut self) {}
    pub fn get_ptr(&self) -> *const Node {
        &self.root
    }
}
