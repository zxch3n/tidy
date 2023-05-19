use std::{
    cell::RefCell, collections::HashSet, hash::BuildHasher, mem::take, ptr::NonNull, rc::Rc,
    thread::panicking,
};

use num::Float;
use tinyset::SetUsize;

use crate::{
    geometry::Coord,
    node::{Link, TidyData, WeakLink},
    Layout, Node,
};

use super::linked_y_list::LinkedYList;

pub struct TidyLayout {
    pub parent_child_margin: Coord,
    pub peer_margin: Coord,
    is_layered: bool,
    /// this is only for layered layout
    depth_to_y: Vec<Coord>,
}

const TEST: usize = 123123231;

impl TidyLayout {
    pub fn new(parent_child_margin: Coord, peer_margin: Coord) -> Self {
        TidyLayout {
            parent_child_margin,
            peer_margin,
            is_layered: false,
            depth_to_y: vec![],
        }
    }

    pub fn new_layered(parent_child_margin: Coord, peer_margin: Coord) -> Self {
        TidyLayout {
            parent_child_margin,
            peer_margin,
            is_layered: true,
            depth_to_y: vec![],
        }
    }
}

struct Contour {
    is_left: bool,
    pub current: Option<Link>,
    modifier_sum: Coord,
}

impl Contour {
    pub fn new(is_left: bool, current: Link) -> Self {
        let borrow = current.borrow();
        let modifier_sum = borrow.tidy().modifier_extreme_right;
        drop(borrow);
        Contour {
            is_left,
            modifier_sum,
            current: Some(current),
        }
    }

    fn link(&self) -> &Link {
        match &self.current {
            Some(node) => &node,
            None => panic!(),
        }
    }

    fn node(&self) -> std::cell::Ref<Node> {
        match &self.current {
            Some(node) => node.borrow(),
            None => panic!(),
        }
    }

    pub fn is_none(&self) -> bool {
        self.current.is_none()
    }

    pub fn left(&self) -> Coord {
        let node = self.node();
        self.modifier_sum + node.relative_x - node.width / 2.
    }

    pub fn right(&self) -> Coord {
        let node = self.node();
        self.modifier_sum + node.relative_x + node.width / 2.
    }

    pub fn bottom(&self) -> Coord {
        match &self.current {
            Some(node) => {
                let node = node.borrow();
                node.y + node.height
            }
            None => 0.,
        }
    }

    pub fn next(&mut self) {
        if let Some(current) = take(&mut self.current) {
            let node = current.borrow_mut();
            if self.is_left {
                if !node.children.is_empty() {
                    self.current = Some(node.children.first().unwrap().borrow().me_rc());
                    let m = self.node().tidy.as_ref().unwrap().modifier_to_subtree;
                    self.modifier_sum += m;
                } else {
                    self.modifier_sum += node.tidy().modifier_thread_left;
                    self.current = node
                        .tidy()
                        .thread_left
                        .as_ref()
                        .map(|x| x.upgrade().unwrap());
                }
            } else if !node.children.is_empty() {
                self.current = Some(node.children.last().unwrap().borrow().me_rc());
                let m = self.node().tidy.as_ref().unwrap().modifier_to_subtree;
                self.modifier_sum += m;
            } else {
                self.modifier_sum += node.tidy().modifier_thread_right;
                self.current = node
                    .tidy()
                    .thread_right
                    .as_ref()
                    .map(|x| x.upgrade().unwrap());
            }
            if self.current.is_some() {
                let node = self.node();
            }
        }
    }
}

impl Node {
    fn set_extreme(&mut self) {
        let self_ptr = self.me_weak();
        let tidy = self.tidy.as_mut().unwrap();
        if self.children.is_empty() {
            tidy.extreme_left = Some(self_ptr.clone());
            tidy.extreme_right = Some(self_ptr);
            tidy.modifier_extreme_left = 0.;
            tidy.modifier_extreme_right = 0.;
        } else {
            let borrow = self.children.first().unwrap().borrow();
            let first = borrow.tidy.as_ref().unwrap();
            tidy.extreme_left = first.extreme_left.clone();
            tidy.modifier_extreme_left = first.modifier_to_subtree + first.modifier_extreme_left;
            let borrow = self.children.last().unwrap().borrow();
            let last = borrow.tidy.as_ref().unwrap();
            tidy.extreme_right = last.extreme_right.clone();
            tidy.modifier_extreme_right = last.modifier_to_subtree + last.modifier_extreme_right;
        }
    }

    fn extreme_left(&mut self) -> Link {
        self.tidy
            .as_mut()
            .unwrap()
            .extreme_left
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }

    fn extreme_right(&mut self) -> Link {
        self.tidy
            .as_mut()
            .unwrap()
            .extreme_right
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }

    fn position_root(&mut self) {
        let first_ = self.children.first().unwrap();
        let first = first_.borrow_mut();
        let first_child_pos = first.relative_x + first.tidy().modifier_to_subtree;
        let last_ = self.children.last().unwrap();
        let last = last_.borrow_mut();
        let last_child_pos = last.relative_x + last.tidy().modifier_to_subtree;
        self.relative_x = (first_child_pos + last_child_pos) / 2.;
        drop(first_);
        drop(first);
        drop(last_);
        drop(last);
        // make modifier_to_subtree + relative_x = 0. so that
        // there will always be collision in `separation()`'s first loop
        self.tidy_mut().modifier_to_subtree = -self.relative_x;
    }

    fn add_child_spacing(&mut self) {
        let mut speed = 0.;
        let mut delta = 0.;
        for child in &mut self.children.iter_mut() {
            let borrow_mut = &mut child.borrow_mut();
            let child = borrow_mut.tidy_mut();
            speed += child.shift_acceleration;
            delta += speed + child.shift_change;
            child.modifier_to_subtree += delta;
            child.shift_acceleration = 0.;
            child.shift_change = 0.;
        }
    }
}

impl TidyLayout {
    fn separate(
        &mut self,
        node: &mut Node,
        child_index: usize,
        mut y_list: LinkedYList,
    ) -> LinkedYList {
        // right contour of the left
        let mut left = Contour::new(false, node.children[child_index - 1].borrow().me_rc());
        // left contour of the right
        let mut right = Contour::new(true, node.children[child_index].borrow().me_rc());
        while !left.is_none() && !right.is_none() {
            if left.bottom() > y_list.bottom() {
                let b = y_list.bottom();
                let top = y_list.pop();
                if top.is_none() {
                    println!(
                        "Err\n\n{}\n\nleft.bottom={}\nyList.bottom={}",
                        node.str(),
                        left.bottom(),
                        b
                    );
                }

                y_list = top.unwrap();
            }

            let dist = left.right() - right.left() + self.peer_margin;
            if dist > 0. {
                // left and right are too close. move right part with distance of dist
                right.modifier_sum += dist;
                self.move_subtree(node, child_index, y_list.index, dist);
            }

            let left_bottom = left.bottom();
            let right_bottom = right.bottom();
            if left_bottom <= right_bottom {
                left.next();
            }
            if left_bottom >= right_bottom {
                right.next();
            }
        }

        if left.is_none() && !right.is_none() {
            self.set_left_thread(node, child_index, right.link(), right.modifier_sum);
        } else if !left.is_none() && right.is_none() {
            self.set_right_thread(node, child_index, left.link(), left.modifier_sum);
        }

        y_list
    }

    fn set_left_thread(
        &mut self,
        node: &mut Node,
        current_index: usize,
        target: &Link,
        modifier: Coord,
    ) {
        let mut first = RefCell::borrow_mut(&node.children[0]);
        let current = node.children[current_index].borrow();
        let diff = modifier
            - first.tidy_mut().modifier_extreme_left
            - first.tidy_mut().modifier_to_subtree;

        let extreme_left = first.extreme_left();
        let borrow_mut = &mut RefCell::borrow_mut(&extreme_left);
        let left_tidy = borrow_mut.tidy_mut();
        left_tidy.thread_left = Some(Rc::downgrade(target));
        left_tidy.modifier_thread_left = diff;
        first.tidy_mut().extreme_left = current.tidy().extreme_left.clone();
        first.tidy_mut().modifier_extreme_left = current.tidy().modifier_extreme_left
            + current.tidy().modifier_to_subtree
            - first.tidy_mut().modifier_to_subtree;
    }

    fn set_right_thread(
        &mut self,
        node: &mut Node,
        current_index: usize,
        target: &Link,
        modifier: Coord,
    ) {
        let mut current = RefCell::borrow_mut(&node.children[current_index]);
        let diff = modifier
            - current.tidy_mut().modifier_extreme_right
            - current.tidy_mut().modifier_to_subtree;
        let extreme_right = current.extreme_right();
        let borrow_mut = &mut RefCell::borrow_mut(&extreme_right);
        let right_tidy = borrow_mut.tidy_mut();
        right_tidy.thread_right = Some(Rc::downgrade(target));
        right_tidy.modifier_thread_right = diff;
        let borrow = &node.children[current_index - 1].borrow();
        let prev = borrow.tidy();
        current.tidy_mut().extreme_right = prev.extreme_right.clone();
        current.tidy_mut().modifier_extreme_right = prev.modifier_extreme_right
            + prev.modifier_to_subtree
            - current.tidy_mut().modifier_to_subtree;
    }

    fn move_subtree(
        &mut self,
        node: &mut Node,
        current_index: usize,
        from_index: usize,
        distance: Coord,
    ) {
        let child = &node.children[current_index];
        let mut borrow_mut = RefCell::borrow_mut(child);
        let child_tidy = borrow_mut.tidy_mut();
        // debug_assert!(distance <= 1e6);
        child_tidy.modifier_to_subtree += distance;

        // distribute extra space to nodes between from_index to current_index
        if from_index != current_index - 1 {
            let index_diff = (current_index - from_index) as Coord;
            RefCell::borrow_mut(&node.children[from_index + 1])
                .tidy_mut()
                .shift_acceleration += distance / index_diff;
            RefCell::borrow_mut(&node.children[current_index])
                .tidy_mut()
                .shift_acceleration -= distance / index_diff;
            RefCell::borrow_mut(&node.children[current_index])
                .tidy_mut()
                .shift_change -= distance - distance / index_diff;
        }
    }

    fn set_y_recursive(&mut self, root: &mut Node) {
        if !self.is_layered {
            root.pre_order_traversal_mut(|node| {
                self.set_y(node);
            });
        } else {
            let depth_to_y = &mut self.depth_to_y;
            depth_to_y.clear();
            let margin = self.parent_child_margin;
            root.bfs_traversal_with_depth_mut(|node, depth| {
                while depth >= depth_to_y.len() {
                    depth_to_y.push(0.);
                }

                if node.parent.is_none() || depth == 0 {
                    node.y = 0.;
                    return;
                }

                let ref_cell = &node.parent().unwrap();
                let parent = ref_cell.borrow_mut();
                depth_to_y[depth] = Float::max(
                    depth_to_y[depth],
                    depth_to_y[depth - 1] + parent.height + self.parent_child_margin,
                );
            });
            root.pre_order_traversal_with_depth_mut(|node, depth| {
                node.y = depth_to_y[depth];
            })
        }
    }

    fn set_y(&mut self, node: &mut Node) {
        node.y = if let Some(parent) = node.parent.as_ref() {
            let parent_bottom = parent.upgrade().unwrap().borrow().bottom();
            parent_bottom + self.parent_child_margin
        } else {
            0.
        };
    }

    fn first_walk(&mut self, node: &mut Node) {
        if node.children.is_empty() {
            node.set_extreme();
            return;
        }

        self.first_walk(&mut *node.children.first().unwrap().borrow_mut());
        let mut borrow = node.children[0].borrow_mut();
        let extreme_right = borrow.extreme_right();
        let mut y_list = LinkedYList::new(0, extreme_right.borrow().bottom());
        drop(borrow);
        drop(extreme_right);
        for i in 1..node.children.len() {
            let mut current_child = node.children.get_mut(i).unwrap().borrow_mut();
            self.first_walk(&mut *current_child);
            let max_y = current_child.extreme_left().borrow().bottom();
            drop(current_child);
            y_list = self.separate(node, i, y_list);
            y_list = y_list.update(i, max_y);
        }

        node.position_root();
        node.set_extreme();
    }

    fn first_walk_with_filter(&mut self, node: &mut Node, set: &SetUsize) {
        if !set.contains(node as *const _ as usize) {
            invalidate_extreme_thread(node);
            return;
        }

        if node.children.is_empty() {
            node.set_extreme();
            return;
        }

        self.first_walk_with_filter(&mut *node.children.first().unwrap().borrow_mut(), set);
        let mut borrow = node.children[0].borrow_mut();
        let mut y_list = LinkedYList::new(0, borrow.extreme_right().borrow().bottom());
        drop(borrow);
        for i in 1..node.children.len() {
            let mut current_child = node.children.get_mut(i).unwrap().borrow_mut();
            current_child.tidy_mut().modifier_to_subtree = -current_child.relative_x;
            self.first_walk_with_filter(&mut *current_child, set);
            let max_y = current_child.extreme_left().borrow().bottom();
            drop(current_child);
            y_list = self.separate(node, i, y_list);
            y_list = y_list.update(i, max_y);
        }

        node.position_root();
        node.set_extreme();
    }

    fn second_walk(&mut self, node: &mut Node, mut mod_sum: Coord) {
        mod_sum += node.tidy_mut().modifier_to_subtree;
        node.x = node.relative_x + mod_sum;
        node.add_child_spacing();

        for child in node.children.iter_mut() {
            self.second_walk(&mut *child.borrow_mut(), mod_sum);
        }
    }

    fn second_walk_with_filter(&mut self, node: &mut Node, mut mod_sum: Coord, set: &SetUsize) {
        mod_sum += node.tidy_mut().modifier_to_subtree;
        let new_x = node.relative_x + mod_sum;
        if (new_x - node.x).abs() < 1e-8 && !set.contains(node as *const _ as usize) {
            return;
        }

        node.x = new_x;
        node.add_child_spacing();

        for child in node.children.iter_mut() {
            self.second_walk_with_filter(&mut *child.borrow_mut(), mod_sum, set);
        }
    }
}

impl Layout for TidyLayout {
    fn layout(&mut self, root: &mut Node) {
        root.pre_order_traversal_mut(init_node);
        self.set_y_recursive(root);
        self.first_walk(root);
        self.second_walk(root, 0.);
    }

    fn parent_child_margin(&self) -> Coord {
        self.parent_child_margin
    }

    fn peer_margin(&self) -> Coord {
        self.peer_margin
    }

    fn partial_layout(&mut self, root: &mut crate::Node, changed: &[WeakLink]) {
        // not implemented for layered
        if self.is_layered {
            self.layout(root);
            return;
        }

        for node in changed.iter() {
            let ref_cell = &node.upgrade().unwrap();
            let mut node = ref_cell.borrow_mut();
            if node.tidy.is_none() {
                init_node(&mut *node);
            }

            // TODO: can be lazy
            self.set_y_recursive(&mut *node);
        }

        let mut set: SetUsize = SetUsize::new();
        for node in changed.iter() {
            set.insert(node.as_ptr() as usize);
            let mut node = node.upgrade().unwrap();
            loop {
                let mut t = node.borrow_mut();
                invalidate_extreme_thread(&mut *t);
                let parent = t.parent();
                if parent.is_none() {
                    break;
                }

                let parent = parent.unwrap();
                set.insert(parent.as_ptr() as usize);
                drop(t);
                node = parent;
            }
        }

        self.first_walk_with_filter(root, &set);
        // TODO: this can be optimized with onscreen detection,
        // then all nodes' absolute x position can be evaluate lazily
        self.second_walk_with_filter(root, 0., &set);
    }
}

fn init_node(node: &mut Node) {
    if node.tidy.is_some() {
        let tidy = node.tidy_mut();
        tidy.extreme_left = None;
        tidy.extreme_right = None;
        tidy.shift_acceleration = 0.;
        tidy.shift_change = 0.;
        tidy.modifier_to_subtree = 0.;
        tidy.modifier_extreme_left = 0.;
        tidy.modifier_extreme_right = 0.;
        tidy.thread_left = None;
        tidy.thread_right = None;
        tidy.modifier_thread_left = 0.;
        tidy.modifier_thread_right = 0.;
    } else {
        node.tidy = Some(Box::new(TidyData {
            extreme_left: None,
            extreme_right: None,
            shift_acceleration: 0.,
            shift_change: 0.,
            modifier_to_subtree: 0.,
            modifier_extreme_left: 0.,
            modifier_extreme_right: 0.,
            thread_left: None,
            thread_right: None,
            modifier_thread_left: 0.,
            modifier_thread_right: 0.,
        }));
    }

    node.x = 0.;
    node.y = 0.;
    node.relative_x = 0.;
    node.relative_y = 0.;
}

fn invalidate_extreme_thread(node: &mut Node) {
    node.set_extreme();
    let extreme_left = &node.extreme_left();
    let borrow_mut = &mut extreme_left.borrow_mut();
    let e_left = borrow_mut.tidy_mut();
    e_left.thread_left = None;
    e_left.thread_right = None;
    e_left.modifier_thread_left = 0.;
    e_left.modifier_thread_right = 0.;
    let extreme_right = &node.extreme_right();
    let borrow_mut = &mut extreme_right.borrow_mut();
    let e_right = borrow_mut.tidy_mut();
    e_right.thread_left = None;
    e_right.thread_right = None;
    e_right.modifier_thread_left = 0.;
    e_right.modifier_thread_right = 0.;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::node::Node;
    #[test]
    fn test_tidy_layout() {
        let mut tidy = TidyLayout::new(1., 1.);
        let mut root = Node::new(0, 1., 1.);
        let first_child = Node::new_with_child(
            1,
            1.,
            1.,
            Node::new_with_child(10, 1., 1., Node::new(100, 1., 1.)),
        );
        root.borrow_mut().append_child(first_child);

        let second = Node::new_with_child(
            2,
            1.,
            1.,
            Node::new_with_child(11, 1., 1., Node::new(101, 1., 1.)),
        );
        root.borrow_mut().append_child(second);
        root.borrow_mut().append_child(Node::new(3, 1., 2.));
        tidy.layout(&mut *root.borrow_mut());
        println!("{}", root.borrow().str());
    }
}
