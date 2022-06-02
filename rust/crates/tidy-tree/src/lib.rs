#![allow(dead_code, unused_imports, unused_variables)]
pub mod geometry;
mod layout;
mod node;
pub use layout::{BasicLayout, Layout};
pub use node::Node;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
