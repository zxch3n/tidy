use rand::{prelude::StdRng, SeedableRng};
use tidy_tree::{BasicLayout, Layout, TidyLayout};
mod gen;

// #[test]
// fn tidy() {
//     let mut rng = StdRng::seed_from_u64(1001);
//     let mut tree = gen::gen_tree(&mut rng, 100_000);
//     let layout = TidyLayout {
//         parent_child_margin: 10.,
//         peer_margin: 10.,
//     };

//     for _ in 0..10 {
//         layout.layout(&mut tree);
//     }
// }

#[test]
fn basic() {
    let mut rng = StdRng::seed_from_u64(1001);
    let mut tree = gen::gen_tree(&mut rng, 100_000);
    let mut layout = BasicLayout {
        parent_child_margin: 10.,
        peer_margin: 10.,
    };

    for _ in 0..1000 {
        layout.layout(&mut tree);
    }
}
