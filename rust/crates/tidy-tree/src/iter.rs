use crate::{
    node::{Link, WeakLink},
    Node,
};

pub struct Iter {
    nodes: Vec<WeakLink>,
}

impl Iterator for Iter {
    type Item = Link;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop().map(|x| x.upgrade().unwrap())
    }
}

fn recursive_iter(node: &Node, nodes: &mut Vec<WeakLink>) {
    nodes.push(node.me_weak());
    for child in node.children.iter() {
        let borrow = child.borrow();
        recursive_iter(&borrow, nodes);
    }
}

impl Node {
    #[inline]
    pub fn iter(&self) -> Iter {
        let mut nodes = Vec::new();
        recursive_iter(self, &mut nodes);
        nodes.reverse();
        Iter { nodes }
    }
}

#[cfg(test)]
mod iter_test {
    use super::*;

    #[test]
    fn test_node_iter() {
        let root_ = Node::new_with_child(0, 1., 1., Node::new(1, 2., 2.));
        let mut root = root_.borrow_mut();
        assert_eq!(root.iter().count(), 2);
        root.append_child(Node::new(2, 3., 3.));
        assert_eq!(root.iter().count(), 3);
        root.append_child(Node::new(3, 3., 3.));
        assert_eq!(root.iter().count(), 4);
        root.children[2]
            .borrow_mut()
            .append_child(Node::new(4, 3., 3.));
        assert_eq!(root.iter().count(), 5);

        drop(root);
        let root = root_.borrow();
        for (i, node) in root.iter().enumerate() {
            assert_eq!(i, node.borrow().id);
        }
    }
}
