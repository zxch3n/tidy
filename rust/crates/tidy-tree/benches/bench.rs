use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rand::prelude::*;
use std::ptr::NonNull;
use tidy_tree::{geometry::Coord, Node};
use tidy_tree::{Layout, TidyLayout};

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

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Box<Node> {
    let mut root = Box::new(gen_node(rng));
    let mut nodes: Vec<NonNull<Node>> = vec![(&mut *root).into()];
    for _ in 0..num {
        let parent_index = rng.gen_range(0..nodes.len());
        let parent = unsafe { nodes[parent_index].as_mut() };
        let node = gen_node(rng);
        parent.append_child(node);
        nodes.push(parent.children.last_mut().unwrap().as_mut().into());
    }

    root
}

pub fn bench(c: &mut Criterion) {
    c.bench_function("test", |run| {
        let mut rng = StdRng::seed_from_u64(0);
        let mut root = gen_tree(&mut rng, 100000);
        let mut layout = TidyLayout::new(10., 10.);
        run.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                layout.layout(&mut root);
            }
            let ans = start.elapsed();
            root.pre_order_traversal_mut(|node| {
                node.clear_layout_info();
            });
            ans
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
