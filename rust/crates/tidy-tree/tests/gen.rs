use std::ptr::NonNull;

use rand::prelude::*;
use tidy_tree::{geometry::Coord, Node};

pub fn gen_node(rng: &mut StdRng) -> Node {
    Node {
        id: rng.gen(),
        width: rng.gen_range(5..50) as Coord,
        height: rng.gen_range(5..50) as Coord,
        x: 0.,
        y: 0.,
        relative_x: 0.,
        relative_y: 0.,
        bbox: Default::default(),
        parent: None,
        children: vec![],
        tidy: None,
    }
}

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Node {
    let root = gen_node(rng);
    let mut nodes: Vec<NonNull<Node>> = vec![(&root).into()];
    let max_chosen_size = 100;
    for _ in 0..num {
        let parent_index = if nodes.len() < max_chosen_size {
            rng.gen_range(0..nodes.len())
        } else {
            rng.gen_range((nodes.len() - max_chosen_size)..nodes.len())
        };
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last().unwrap().as_ref().into());
    }

    root
}
