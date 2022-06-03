#![allow(dead_code, unused_imports, unused_variables)]
pub mod geometry;
mod layout;
mod node;
use std::{any::Any, collections::HashMap, ptr::NonNull};

use geometry::Coord;
use layout::BoundingBox;
pub use layout::{BasicLayout, Layout};
pub use node::Node;

pub struct TidyTree {
    root: Node,
    layout: Box<dyn Layout>,
    map: HashMap<usize, NonNull<Node>>,
}

impl TidyTree {
    pub fn with_basic_layout() -> Self {
        TidyTree {
            root: Default::default(),
            layout: Box::new(BasicLayout {
                parent_child_margin: 10.,
                peer_margin: 10.,
            }),
            map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.id == usize::MAX
    }

    pub fn add_node(&mut self, id: usize, width: Coord, height: Coord, parent_id: usize) {
        let node = Node::new(id, width, height);
        if self.is_empty() || parent_id == usize::MAX {
            self.root = node;
            self.map.insert(id, (&self.root).into());
            return;
        }

        let mut parent = *self.map.get(&parent_id).unwrap();
        let parent = unsafe { parent.as_mut() };

        let ptr = parent.append_child(node);
        self.map.insert(id, ptr);
    }

    pub fn data(&mut self, id: &[usize], width: &[Coord], height: &[Coord], parent_id: &[usize]) {
        for (i, &id) in id.iter().enumerate() {
            let width = width[i];
            let height = height[i];
            let parent_id = parent_id[i];
            self.add_node(id, width, height, parent_id);
        }
    }

    pub fn layout(&mut self) {
        if self.is_empty() {
            return;
        }

        self.layout.layout(&mut self.root);
    }

    pub fn get_pos(&self) -> Vec<Coord> {
        let mut ans = vec![];
        for (id, node) in self.map.iter() {
            let node = unsafe { node.as_ref() };
            ans.push((*id) as Coord);
            ans.push(node.x);
            ans.push(node.y);
        }

        ans
    }
}

pub const NULL_ID: usize = usize::MAX;
