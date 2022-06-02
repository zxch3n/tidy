use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Node<Meta> {
    pub width: isize,
    pub height: isize,
    pub x: isize,
    pub y: isize,
    pub meta: Meta,
    pub parent: Option<NonNull<Node<Meta>>>,
    pub children: Vec<Box<Node<Meta>>>,
}

impl<Meta: Default> Default for Node<Meta> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            children: vec![],
            parent: None,
            meta: Default::default(),
        }
    }
}

impl<Meta: Default> Node<Meta> {
    pub fn new(width: isize, height: isize) -> Self {
        Node {
            width,
            height,
            meta: Default::default(),
            x: 0,
            y: 0,
            children: vec![],
            parent: None,
        }
    }
}

impl<Meta> Node<Meta> {
    pub fn append_child(&mut self, mut child: Self) {
        child.parent = Some(self.into());
        self.children.push(Box::new(child));
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn post_order_traversal<F>(&self, mut f: F)
    where
        F: FnMut(&Node<Meta>),
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
        F: FnMut(&mut Node<Meta>),
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
        F: FnMut(&Node<Meta>),
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
        F: FnMut(&mut Node<Meta>),
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
