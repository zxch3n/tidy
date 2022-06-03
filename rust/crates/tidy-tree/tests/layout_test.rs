use std::{fmt::Debug, ptr::NonNull};

mod aesthetic_rules;
use rand::prelude::*;
use tidy_tree::{geometry::Coord, BasicLayout, Layout, Node};

pub fn test_layout<D: Default + Debug + Clone>(layout: &mut dyn Layout<Meta = D>) {
    let mut rng = StdRng::seed_from_u64(101);
    let mut tree = gen_tree::<D>(&mut rng, 1000);
    layout.layout(&mut tree);
    aesthetic_rules::assert_no_overlap_nodes(&tree);
    aesthetic_rules::assert_no_crossed_lines(&tree);
    aesthetic_rules::assert_symmetric(&tree, layout);
    aesthetic_rules::check_nodes_order(&tree);
    aesthetic_rules::check_y_position_in_same_level(&tree);
}

pub fn gen_tree<Meta: Default>(rng: &mut StdRng, num: usize) -> Node<Meta> {
    let root = gen_node(rng);
    let mut nodes: Vec<NonNull<Node<Meta>>> = vec![(&root).into()];
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last().unwrap().as_ref().into());
    }

    root
}

fn gen_node<Meta: Default>(rng: &mut StdRng) -> Node<Meta> {
    Node {
        width: rng.gen_range(1..10) as Coord,
        height: rng.gen_range(1..10) as Coord,
        x: 0.,
        y: 0.,
        meta: Default::default(),
        parent: None,
        children: vec![],
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
