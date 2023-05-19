#![feature(test)]

mod aesthetic_rules;
mod gen;

use std::time::Instant;

use rand::{prelude::StdRng, SeedableRng};
use tidy_tree::{BasicLayout, Layout, TidyLayout};
extern crate test;
use test::Bencher;

#[bench]
fn bench_tidy_layout_chart(_bench: &mut Bencher) {
    let mut layout = TidyLayout::new(10., 10.);

    let mut rng = StdRng::seed_from_u64(1001);
    let mut out = vec![];
    let (root, mut nodes) = gen::prepare_tree(&mut rng);
    for num in (1000..500_000).step_by(1000) {
        gen::insert_new_to_tree(&mut rng, 1000, &mut nodes);
        let start = Instant::now();
        layout.layout(&mut root.borrow_mut());
        let time = Instant::now().duration_since(start);
        out.push((num, time.as_micros()));

        if num % 100_000 == 0 {
            println!("{}", num);
            assert!(root.borrow().x == 0.);
        }
    }

    for (num, time) in out {
        println!("{} {}", num, time);
    }
}

#[bench]
fn bench_naive_layout_chart(_bench: &mut Bencher) {
    let mut layout = BasicLayout {
        parent_child_margin: 10.,
        peer_margin: 10.,
    };

    let mut rng = StdRng::seed_from_u64(1001);
    let mut out = vec![];
    let (mut root, mut nodes) = gen::prepare_tree(&mut rng);
    for num in (1000..500_000).step_by(1000) {
        gen::insert_new_to_tree(&mut rng, 1000, &mut nodes);
        let start = Instant::now();
        layout.layout(&mut root.borrow_mut());
        let time = Instant::now().duration_since(start);
        out.push((num, time.as_micros()));

        if num % 100_000 == 0 {
            println!("{}", num);
            assert!(root.borrow().x == 0.);
        }
    }

    for (num, time) in out {
        println!("{} {}", num, time);
    }
}
