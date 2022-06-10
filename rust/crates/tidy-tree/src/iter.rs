use crate::Node;

struct Iter<'a> {
    node: &'a Node,
    slot_stack: Vec<usize>,
    finished: bool,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let ans = self.node;
        if self.node.children.len() > 0 {
            self.node = &self.node.children[0];
            self.slot_stack.push(0);
            return Some(ans);
        } else if self.node.parent.is_some() {
            let mut parent = unsafe { self.node.parent.unwrap().as_ref() };
            let mut index_slot = self.slot_stack.pop().unwrap();
            while parent.children.len() <= index_slot + 1
                && self.slot_stack.len() > 0
                && parent.parent.is_some()
            {
                parent = unsafe { parent.parent.unwrap().as_ref() };
                index_slot = self.slot_stack.pop().unwrap();
            }

            if parent.children.len() > index_slot + 1 {
                self.node = &parent.children[index_slot + 1];
                self.slot_stack.push(index_slot + 1);
                return Some(ans);
            } else {
                self.finished = true;
                return Some(ans);
            }
        }

        return Some(ans);
    }
}

impl Node {
    #[inline]
    fn iter(&self) -> Iter {
        Iter {
            node: self,
            slot_stack: vec![],
            finished: false,
        }
    }
}

#[cfg(test)]
mod iter_test {
    use super::*;

    #[test]
    fn test_node_iter() {
        let mut root = Node::new_with_child(0, 1., 1., Node::new(1, 2., 2.));
        assert_eq!(root.iter().count(), 2);
        root.append_child(Node::new(2, 3., 3.));
        assert_eq!(root.iter().count(), 3);
        for (i, node) in root.iter().enumerate() {
            assert_eq!(i, node.id);
        }
    }
}
