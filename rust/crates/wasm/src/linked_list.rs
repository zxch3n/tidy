use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[wasm_bindgen]
pub struct LinkedList {
    head: Option<Box<Node>>,
    len: usize,
}

#[wasm_bindgen]
impl LinkedList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: None });
        match self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(ref mut _node) => {
                let mut node: *mut Box<Node> = _node;
                unsafe {
                    while let Some(next) = &mut (*node).next {
                        node = next;
                    }

                    (*node).next = Some(new_node);
                }
            }
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }
}

#[cfg(test)]
mod test {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_link() {
        let mut link: LinkedList = LinkedList::new();
        link.push(1);
        link.push(2);
        link.push(3);
        link.push(4);
        assert!(link.pop() == Some(1));
        assert!(link.pop() == Some(2));
        assert!(link.pop() == Some(3));
        assert!(link.pop() == Some(4));
        assert!(link.pop() == None);
    }
}
