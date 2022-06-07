use std::ptr::NonNull;

use crate::{geometry::Coord, layout::BoundingBox};

#[derive(Debug, Clone)]
pub struct TidyData {
    pub thread_left: Option<NonNull<Node>>,
    pub thread_right: Option<NonNull<Node>>,
    /// ```
    /// this.extreme_left == this.thread_left.extreme_left ||
    /// this.extreme_left == this.children[0].extreme_left
    /// ```
    pub extreme_left: Option<NonNull<Node>>,
    /// ```
    /// this.extreme_right == this.thread_right.extreme_right ||
    /// this.extreme_right == this.children[-1].extreme_right
    /// ```
    pub extreme_right: Option<NonNull<Node>>,

    /// Cached change of x position.
    pub shift_acceleration: Coord,
    /// Cached change of x position
    pub shift_change: Coord,

    /// this.x = parent.x + modifier_to_subtree
    pub modifier_to_subtree: Coord,
    /// this.x + modifier_thread_left == thread_left.x
    pub modifier_thread_left: Coord,
    /// this.x + modifier_thread_right == thread_right.x
    pub modifier_thread_right: Coord,
    /// this.x + modifier_extreme_left == extreme_left.x
    pub modifier_extreme_left: Coord,
    /// this.x + modifier_extreme_right == extreme_right.x
    pub modifier_extreme_right: Coord,
}

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
    pub tidy: Option<Box<TidyData>>,
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
            tidy: None,
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
            tidy: None,
        }
    }
}

impl Node {
    pub fn bottom(&self) -> Coord {
        self.height + self.y
    }

    pub fn tidy_mut(&mut self) -> &mut TidyData {
        self.tidy.as_mut().unwrap()
    }

    pub fn tidy(&self) -> &TidyData {
        self.tidy.as_ref().unwrap()
    }

    pub fn append_child(&mut self, mut child: Self) -> NonNull<Self> {
        child.parent = Some(self.into());
        let boxed = Box::new(child);
        let ptr = boxed.as_ref().into();
        self.children.push(boxed);
        ptr
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x - self.width / 2. < other.x + other.width / 2.
            && self.x + self.width / 2. > other.x - other.width / 2.
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
