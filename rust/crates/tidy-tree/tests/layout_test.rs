use std::ptr::NonNull;

mod aesthetic_rules;
use rand::prelude::*;
use tidy_tree::{geometry::Coord, BasicLayout, Layout, Node};

pub fn test_layout(layout: &mut dyn Layout) {
    let mut rng = StdRng::seed_from_u64(101);
    for _ in 0..100 {
        let mut tree = gen_tree(&mut rng, 100);
        layout.layout(&mut tree);
        aesthetic_rules::assert_no_overlap_nodes(&tree);
        aesthetic_rules::assert_no_crossed_lines(&tree);
        aesthetic_rules::assert_symmetric(&tree, layout);
        aesthetic_rules::check_nodes_order(&tree);
        aesthetic_rules::check_y_position_in_same_level(&tree);
        aesthetic_rules::assert_parent_centered(&tree);
    }
}

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Node {
    let root = gen_node(rng);
    let mut nodes: Vec<NonNull<Node>> = vec![(&root).into()];
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last().unwrap().as_ref().into());
    }

    root
}

fn gen_node(rng: &mut StdRng) -> Node {
    Node {
        id: rng.gen(),
        width: rng.gen_range(1..10) as Coord,
        height: rng.gen_range(1..10) as Coord,
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

#[test]
fn test_basic_layout() {
    let mut layout = BasicLayout {
        parent_child_margin: 10.,
        peer_margin: 10.,
    };
    test_layout(&mut layout);
}
