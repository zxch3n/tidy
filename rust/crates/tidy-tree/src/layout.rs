use crate::node::Node;
use std::ptr::NonNull;
mod basic_layout;
mod linked_y_list;
mod tidy_layout;
pub use basic_layout::{BasicLayout, BoundingBox};
pub use tidy_layout::TidyLayout;

pub trait Layout {
    fn layout(&self, root: &mut Node);
    fn partial_layout(&self, root: &mut Node, changed: &[NonNull<Node>]);
}
