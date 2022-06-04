use std::ptr::NonNull;

use crate::{geometry::Coord, layout::BoundingBox};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub width: Coord,
    pub height: Coord,
    pub x: Coord,
    pub y: Coord,
    /// node x position relative to its parent
    pub relative_x: Coord,
    /// node y position relative to its parent
    pub relative_y: Coord,
    pub bbox: BoundingBox,
    pub parent: Option<NonNull<Node>>,
    /// Children need boxing to get a stable addr in the heap
    pub children: Vec<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: usize::MAX,
            width: 0.,
            height: 0.,
            x: 0.,
            y: 0.,
            relative_x: 0.,
            relative_y: 0.,
            children: vec![],
            parent: None,
            bbox: Default::default(),
        }
    }
}

impl Node {
    pub fn new(id: usize, width: Coord, height: Coord) -> Self {
        Node {
            id,
            width,
            height,
            bbox: Default::default(),
            x: 0.,
            y: 0.,
            relative_x: 0.,
            relative_y: 0.,
            children: vec![],
            parent: None,
        }
    }
}

impl Node {
    pub fn append_child(&mut self, mut child: Self) -> NonNull<Self> {
        child.parent = Some(self.into());
        let boxed = Box::new(child);
        let ptr = boxed.as_ref().into();
        self.children.push(boxed);
        ptr
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn post_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&Node),
    {
        let mut stack: Vec<(NonNull<Self>, bool)> = vec![(self.into(), true)];
        while let Some((mut node_ptr, is_first)) = stack.pop() {
            let node = unsafe { node_ptr.as_mut() };
            if !is_first {
                f(node);
                continue;
            }

            stack.push((node_ptr, false));
            for child in node.children.iter_mut() {
                stack.push((child.as_ref().into(), true));
            }
        }
    }

    pub fn post_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node),
    {
        let mut stack: Vec<(NonNull<Self>, bool)> = vec![(self.into(), true)];
        while let Some((mut node_ptr, is_first)) = stack.pop() {
            let node = unsafe { node_ptr.as_mut() };
            if !is_first {
                f(node);
                continue;
            }

            stack.push((node_ptr, false));
            for child in node.children.iter_mut() {
                stack.push((child.as_ref().into(), true));
            }
        }
    }

    pub fn pre_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&Node),
    {
        let mut stack: Vec<NonNull<Self>> = vec![self.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter_mut() {
                stack.push(child.as_ref().into());
            }
        }
    }

    pub fn pre_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node),
    {
        let mut stack: Vec<NonNull<Self>> = vec![self.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter_mut() {
                stack.push(child.as_ref().into());
            }
        }
    }
}
