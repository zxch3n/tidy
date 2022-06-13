use rand::{prelude::StdRng, SeedableRng};
use wasm::Tidy;
use web_sys::{self, Performance};

mod gen;
use gen::gen_tree;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn bench_tidy() {
    let performance = get_performance();
    let mut rng = StdRng::seed_from_u64(1001);
    let tree = gen_tree(&mut rng, 100_000);
    let mut layout = Tidy::with_tidy_layout(10., 10.);
    tree.pre_order_traversal(|node| {
        if node.parent.is_some() {
            layout.add_node(
                node.id,
                node.width,
                node.height,
                unsafe { node.parent.unwrap().as_ref() }.id,
            );
        } else {
            layout.add_node(node.id, node.width, node.height, Tidy::null_id());
        }
    });
    let start = performance.now();
    let times = 8;
    for _ in 0..times {
        layout.layout();
    }
    console_log!("{}ms", (performance.now() - start) / (times as f64));
}

fn get_performance() -> Performance {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    performance
}
