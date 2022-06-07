use num::Float;

use super::Layout;
use crate::{geometry::Coord, node::Node};
use std::cmp::{max, min};

/// <img src="https://i.ibb.co/BLCfz0g/image.png" width="300" alt="Relative position"/>
///
/// Relative position illustration
pub struct BasicLayout {
    pub parent_child_margin: Coord,
    pub peer_margin: Coord,
}

/// <img src="https://i.ibb.co/BLCfz0g/image.png" width="300" alt="Relative position"/>
///
/// Relative position illustration
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub total_width: Coord,
    pub total_height: Coord,
    /// bounding box left position
    pub shift_x: Coord,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            total_height: 0.,
            total_width: 0.,
            shift_x: 0.,
        }
    }
}

impl Layout for BasicLayout {
    fn layout(&mut self, root: &mut Node) {
        root.pre_order_traversal_mut(|node| {
            node.tidy = None;
            node.x = 0.;
            node.y = 0.;
            node.relative_x = 0.;
            node.relative_y = 0.;
        });
        root.post_order_traversal_mut(|node| {
            self.update_meta(node);
        });
        root.pre_order_traversal_mut(|node| {
            if let Some(parent) = node.parent {
                let parent = unsafe { parent.as_ref() };
                node.x = parent.x + node.relative_x;
                node.y = parent.y + node.relative_y;
            }
        });
    }

    fn partial_layout(&mut self, root: &mut Node, changed: &[std::ptr::NonNull<Node>]) {
        todo!()
    }
}

impl BasicLayout {
    fn update_meta(&mut self, node: &mut Node) {
        node.bbox = BoundingBox {
            total_height: node.height,
            total_width: node.width,
            shift_x: -node.width / 2.,
        };
        let children: *mut _ = &mut node.children;
        let children = unsafe { &mut *children };
        let n = children.len() as Coord;
        if n > 0. {
            let mut total_width: Coord = 0.;
            for child in children.iter() {
                total_width += child.bbox.total_width;
            }

            total_width += (n - 1.) * self.peer_margin;
            let mut relative_x = 0.;
            let mut max_height = 0.;
            let n = children.len();
            for (i, child) in children.iter_mut().enumerate() {
                child.relative_y = node.height + self.parent_child_margin;
                relative_x += -child.bbox.shift_x;
                child.relative_x = relative_x;
                relative_x += child.bbox.total_width + child.bbox.shift_x + self.peer_margin;
                max_height = Float::max(child.bbox.total_height, max_height);
            }

            let shift_x = -total_width / 2.;
            for child in children.iter_mut() {
                child.relative_x += shift_x;
            }

            node.bbox.total_width = total_width;
            node.bbox.total_height = node.height + self.parent_child_margin + max_height;
            node.bbox.shift_x = shift_x;
        }
    }
}

#[cfg(test)]
mod basic_layout_test {
    use super::{BasicLayout, BoundingBox};
    use crate::{layout::Layout, Node};

    #[test]
    fn easy_test_0() {
        let mut root = Node::new(0, 10., 10.);
        root.append_child(Node::new(1, 10., 10.));
        let mut second = Node::new(2, 10., 10.);
        second.append_child(Node::new(3, 10., 10.));
        root.append_child(second);
        root.append_child(Node::new(4, 10., 10.));
        let mut layout = BasicLayout {
            parent_child_margin: 10.,
            peer_margin: 5.,
        };
        layout.layout(&mut root);
        println!("{:#?}", root);
    }
}
