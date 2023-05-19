use crate::{
    geometry::Coord,
    node::{Node, WeakLink},
};
use std::ptr::NonNull;
mod basic_layout;
mod linked_y_list;
mod tidy_layout;
pub use basic_layout::{BasicLayout, BoundingBox};
pub use tidy_layout::TidyLayout;

pub trait Layout {
    fn layout(&mut self, root: &mut Node);
    fn partial_layout(&mut self, root: &mut Node, changed: &[WeakLink]);
    fn parent_child_margin(&self) -> Coord;
    fn peer_margin(&self) -> Coord;
}
