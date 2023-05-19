use rand::{prelude::StdRng, SeedableRng};
use wasm::Tidy;
use web_sys::{self, Performance};

mod gen;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// this is unreliable
/// we cannot get accurate performance.now from browser
#[wasm_bindgen_test]
fn bench_tidy() {
    let performance = get_performance();
    let mut rng = StdRng::seed_from_u64(1001);
    let mut out = vec![];
    for num in (1000..110_000).step_by(1000) {
        let mut layout = Tidy::with_tidy_layout(10., 10.);
        let tree = gen::gen_tree(&mut rng, num);
        let tree = tree.borrow();
        tree.pre_order_traversal(|node| {
            if node.parent.is_some() {
                layout.add_node(
                    node.id,
                    node.width,
                    node.height,
                    node.parent().unwrap().borrow().id,
                );
            } else {
                layout.add_node(node.id, node.width, node.height, Tidy::null_id());
            }
        });
        let start = performance.now();
        for _ in 0..10 {
            layout.layout();
        }
        let time = performance.now() - start;
        out.push((num, time * 100.));
        drop(tree);
    }

    for (num, time) in out {
        console_log!("{} {}", num, time);
    }
}

fn get_performance() -> Performance {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    performance
}
