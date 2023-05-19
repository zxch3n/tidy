use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{geometry::Coord, layout::BoundingBox};

#[derive(Debug)]
pub struct TidyData {
    pub thread_left: Option<WeakLink>,
    pub thread_right: Option<WeakLink>,
    /// ```text
    /// this.extreme_left == this.thread_left.extreme_left ||
    /// this.extreme_left == this.children[0].extreme_left
    /// ```
    pub extreme_left: Option<WeakLink>,
    /// ```text
    /// this.extreme_right == this.thread_right.extreme_right ||
    /// this.extreme_right == this.children[-1].extreme_right
    /// ```
    pub extreme_right: Option<WeakLink>,

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

pub type Link = Rc<RefCell<Node>>;
pub type WeakLink = Weak<RefCell<Node>>;

#[derive(Debug)]
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
    pub parent: Option<WeakLink>,
    /// Children need boxing to get a stable addr in the heap
    pub children: Vec<Link>,
    pub tidy: Option<Box<TidyData>>,
    pub this: WeakLink,
}

impl Node {
    pub fn new(id: usize, width: Coord, height: Coord) -> Link {
        Rc::new_cyclic(|me| {
            RefCell::new(Self {
                id,
                width,
                height,
                x: 0.,
                y: 0.,
                relative_x: 0.,
                relative_y: 0.,
                bbox: Default::default(),
                parent: None,
                children: Vec::new(),
                tidy: None,
                this: me.clone(),
            })
        })
    }

    pub fn clone_subtree(&self) -> Link {
        let mut visited: HashMap<usize, Link> = Default::default();
        let cloned_root = self.clone_data();
        visited.insert(self.id, cloned_root.clone());
        self.pre_order_traversal(|node| {
            if visited.contains_key(&node.id) {
                return;
            }

            let ref_cell = &node.parent().unwrap();
            let parent = ref_cell.borrow();
            let parent_id = parent.id;
            let mut cloned_parent = visited.get(&parent_id).unwrap().borrow_mut();
            let cloned_node = node.clone_data();
            cloned_parent.append_child(cloned_node.clone());
            drop(cloned_parent);
            visited.insert(node.id, cloned_node);
        });

        cloned_root
    }

    fn clone_data(&self) -> Link {
        Rc::new_cyclic(|me| {
            RefCell::new(Self {
                id: self.id,
                width: self.width,
                height: self.height,
                x: self.x,
                y: self.x,
                relative_x: self.relative_x,
                relative_y: self.relative_y,
                bbox: self.bbox.clone(),
                parent: None,
                children: Vec::new(),
                tidy: None,
                this: me.clone(),
            })
        })
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut current = self.parent.clone();
        while let Some(parent) = current {
            let ref_cell = parent.upgrade().unwrap();
            current = ref_cell.borrow().parent.clone();
            depth += 1;
        }

        depth
    }

    pub fn parent(&self) -> Option<Link> {
        self.parent.as_ref().map(|node| node.upgrade().unwrap())
    }

    pub fn bottom(&self) -> Coord {
        self.height + self.y
    }

    pub fn tidy_mut(&mut self) -> &mut TidyData {
        self.tidy.as_mut().unwrap()
    }

    pub fn tidy(&self) -> &TidyData {
        self.tidy.as_ref().unwrap()
    }

    pub fn me_weak(&self) -> WeakLink {
        self.this.clone()
    }

    pub fn me_rc(&self) -> Link {
        self.this.upgrade().unwrap()
    }

    fn reset_parent_link_of_children(&mut self) {
        if self.children.is_empty() {
            return;
        }

        for child in self.children.iter_mut() {
            child.borrow_mut().parent = Some(self.this.clone());
        }
    }

    pub fn append_child(&mut self, child: Link) -> WeakLink {
        let mut child_ = child.borrow_mut();
        child_.parent = Some(self.me_weak());
        child_.reset_parent_link_of_children();
        let weak = child_.me_weak();
        drop(child_);
        self.children.push(child);
        weak
    }

    pub fn new_with_child(id: usize, width: Coord, height: Coord, child: Link) -> Link {
        let node = Node::new(id, width, height);
        node.borrow_mut().append_child(child);
        node
    }

    pub fn new_with_children(id: usize, width: Coord, height: Coord, children: Vec<Link>) -> Link {
        let node = Node::new(id, width, height);
        let mut node_ = node.borrow_mut();
        for child in children {
            node_.append_child(child);
        }
        drop(node_);
        node
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
        let mut stack: Vec<(Link, bool)> = vec![];
        for child in self.children.iter() {
            stack.push((child.borrow().this.upgrade().unwrap(), true));
        }
        while let Some((node_ptr, is_first)) = stack.pop() {
            let node: &Node = &mut node_ptr.borrow();
            if !is_first {
                f(node);
                continue;
            }

            stack.push((node_ptr.clone(), false));
            for child in node.children.iter() {
                stack.push((child.borrow().this.upgrade().unwrap(), true));
            }
        }
        f(self);
    }

    pub fn post_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node),
    {
        let mut stack: Vec<(Link, bool)> = vec![];
        for child in self.children.iter_mut() {
            stack.push((child.borrow().this.upgrade().unwrap(), true));
        }
        while let Some((node_ptr, is_first)) = stack.pop() {
            let mut node = node_ptr.borrow_mut();
            if !is_first {
                f(&mut node);
                continue;
            }

            stack.push((node_ptr.clone(), false));
            for child in node.children.iter_mut() {
                stack.push((child.borrow().this.upgrade().unwrap(), true));
            }
        }

        f(self);
    }

    pub fn pre_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&Node),
    {
        let mut stack: Vec<Link> = vec![];
        f(self);
        for child in self.children.iter() {
            stack.push(child.borrow().this.upgrade().unwrap());
        }
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            f(&node);
            for child in node.children.iter() {
                stack.push(child.borrow().this.upgrade().unwrap());
            }
        }
    }

    pub fn pre_order_traversal_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node),
    {
        let mut stack: Vec<Link> = vec![];
        f(self);
        for child in self.children.iter_mut() {
            stack.push(child.borrow().this.upgrade().unwrap());
        }
        while let Some(node) = stack.pop() {
            let mut node = node.borrow_mut();
            f(&mut node);
            for child in node.children.iter_mut() {
                stack.push(child.borrow().this.upgrade().unwrap());
            }
        }
    }

    pub fn bfs_traversal_with_depth_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node, usize),
    {
        let mut queue: VecDeque<(Link, usize)> = VecDeque::new();
        queue.push_back((self.me_rc(), 0));
        while let Some((node, depth)) = queue.pop_front() {
            let mut node = node.borrow_mut();
            f(&mut node, depth);
            for child in node.children.iter_mut() {
                queue.push_back((child.borrow().me_rc(), depth + 1));
            }
        }
    }

    pub fn remove_child(&mut self, id: usize) {
        let pos = self.children.iter().position(|node| node.borrow().id == id);
        if let Some(index) = pos {
            self.children.remove(index);
        }
    }

    pub fn pre_order_traversal_with_depth_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Node, usize),
    {
        let mut stack: Vec<(Link, usize)> = vec![];
        f(self, 0);
        for child in self.children.iter_mut() {
            stack.push((child.borrow().me_rc(), 1));
        }
        while let Some((node, depth)) = stack.pop() {
            let mut node = node.borrow_mut();
            f(&mut node, depth);
            for child in node.children.iter_mut() {
                stack.push((child.borrow().me_rc(), depth + 1));
            }
        }
    }

    pub fn str(&self) -> String {
        let mut s = String::new();
        if self.tidy.is_some() {
            s.push_str(&format!(
                "x: {}, y: {}, width: {}, height: {}, rx: {}, mod: {}, id: {}\n",
                self.x,
                self.y,
                self.width,
                self.height,
                self.relative_x,
                self.tidy().modifier_to_subtree,
                self.id
            ));
        } else {
            s.push_str(&format!(
                "x: {}, y: {}, width: {}, height: {}, rx: {}, id: {}\n",
                self.x, self.y, self.width, self.height, self.relative_x, self.id
            ));
        }
        for child in self.children.iter() {
            for line in child.borrow().str().split('\n') {
                if line.is_empty() {
                    continue;
                }

                s.push_str(&format!("    {}\n", line));
            }
        }

        s
    }

    pub fn clear_layout_info(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.tidy = None;
        self.relative_x = 0.;
        self.relative_y = 0.;
        self.bbox = Default::default();
    }
}
