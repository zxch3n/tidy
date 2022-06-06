use crate::node::Node;
use std::ptr::NonNull;
mod basic_layout;
mod linked_y_list;
mod tidy_layout;
pub use basic_layout::{BasicLayout, BoundingBox};

pub trait Layout {
    fn layout(&mut self, root: &mut Node);
    fn partial_layout(&mut self, root: &mut Node, changed: &[NonNull<Node>]);
}
