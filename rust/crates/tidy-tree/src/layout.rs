use std::ptr::NonNull;

use crate::node::Node;

pub trait Layout {
    type Meta;
    fn layout(&mut self, root: &mut Node<Self::Meta>);
    fn partial_layout(
        &mut self,
        root: &mut Node<Self::Meta>,
        changed: &[NonNull<Node<Self::Meta>>],
    );
}

mod basic_layout;
pub use basic_layout::BasicLayout;
