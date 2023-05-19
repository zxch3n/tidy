use rand::prelude::*;
use tidy_tree::{geometry::Coord, Link, Node};

pub fn gen_node(rng: &mut StdRng) -> Link {
    Node::new(
        rng.gen(),
        rng.gen_range(5..50) as Coord,
        rng.gen_range(5..50) as Coord,
    )
}

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Link {
    let root = gen_node(rng);
    let mut nodes: Vec<Link> = vec![root.clone()];
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = nodes[parent_index].clone();
        let node = gen_node(rng);
        parent.borrow_mut().append_child(node);
        nodes.push(
            parent
                .borrow_mut()
                .children
                .last_mut()
                .unwrap()
                .borrow()
                .me_rc(),
        );
    }

    root
}

#[allow(unused)]
pub fn prepare_tree(rng: &mut StdRng) -> (Link, Vec<Link>) {
    let mut root = gen_node(rng);
    let nodes: Vec<Link> = vec![root.clone()];
    (root, nodes)
}

#[allow(unused)]
pub fn insert_new_to_tree(rng: &mut StdRng, num: usize, nodes: &mut Vec<Link>) {
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = nodes[parent_index].clone();
        let mut parent = parent.borrow_mut();
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last().unwrap().borrow().me_rc());
    }
}
