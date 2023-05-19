use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rand::prelude::*;
use std::ptr::NonNull;
use tidy_tree::{geometry::Coord, Node};
use tidy_tree::{BasicLayout, Layout, Link, TidyLayout};

pub fn gen_node(rng: &mut StdRng) -> Link {
    Node::new(
        rng.gen(),
        rng.gen_range(5..50) as Coord,
        rng.gen_range(5..50) as Coord,
    )
}

pub fn gen_tree(rng: &mut StdRng, num: usize) -> Link {
    let mut root = gen_node(rng);
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

pub fn bench(c: &mut Criterion) {
    c.bench_function("tidy 100k", |run| {
        let mut rng = StdRng::seed_from_u64(0);
        let mut root = gen_tree(&mut rng, 100_000);
        let mut layout = TidyLayout::new(10., 10.);
        run.iter_custom(|iters| {
            let start = std::time::Instant::now();
            let mut borrow_mut = root.borrow_mut();
            for _ in 0..iters {
                layout.layout(&mut borrow_mut);
            }
            let ans = start.elapsed();
            borrow_mut.pre_order_traversal_mut(|node| {
                node.clear_layout_info();
            });
            ans
        });
    });

    c.bench_function("naive 100k", |run| {
        let mut rng = StdRng::seed_from_u64(0);
        let root = gen_tree(&mut rng, 100_000);
        let mut layout = BasicLayout::new(10., 10.);
        run.iter_custom(|iters| {
            let start = std::time::Instant::now();
            let mut borrow_mut = root.borrow_mut();
            for _ in 0..iters {
                layout.layout(&mut borrow_mut);
            }
            let ans = start.elapsed();
            borrow_mut.pre_order_traversal_mut(|node| {
                node.clear_layout_info();
            });
            ans
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
