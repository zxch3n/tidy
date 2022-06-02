use std::{fmt::Debug, ptr::NonNull};

use tidy_tree::{geometry::*, Layout, Node};

pub fn assert_no_overlap_nodes<D: Debug>(root: &Node<D>) {
    let mut nodes: Vec<NonNull<Node<D>>> = vec![];
    root.post_order_traversal(|node| {
        for other in nodes.iter() {
            let other = unsafe { other.as_ref() };
            if node.intersects(other) {
                let msg = format!("{:#?} and {:#?} overlap", node, other);
                panic!("{}", msg);
            }
        }

        nodes.push(node.into());
    });
}

pub fn assert_no_crossed_lines<D: Debug>(root: &Node<D>) {
    let mut lines: Vec<Line> = vec![];
    root.post_order_traversal(|node| {
        for child in node.children.iter() {
            let line = Line {
                from: Point {
                    x: node.x,
                    y: node.y,
                },
                to: Point {
                    x: child.x,
                    y: child.y,
                },
            };
            for other in lines.iter() {
                assert!(
                    !line.intersect(other) || line.connected_to(other),
                    "{:#?} and {:#?} intersect",
                    line,
                    other
                );
            }

            lines.push(line);
        }
    });
}

pub fn assert_symmetric<D: Debug + Clone>(root: &Node<D>, layout: &mut dyn Layout<Meta = D>) {
    let mut mirrored = mirror(root);
    layout.layout(&mut mirrored);
    let mut point_origin: Vec<isize> = vec![];
    let mut point_mirrored: Vec<isize> = vec![];
    root.pre_order_traversal(|node| {
        if let Some(parent) = node.parent {
            let parent = unsafe { parent.as_ref() };
            point_origin.push(node.x - parent.x);
        }
    });
    pre_order_traversal_rev(&mirrored, |node| {
        if let Some(parent) = node.parent {
            let parent = unsafe { parent.as_ref() };
            point_mirrored.push(-node.x + parent.x);
        }
    });
    // println!("{:#?}", root);
    // println!("{:#?}", mirrored);

    assert_eq!(point_origin.len(), point_mirrored.len());
    for i in 0..point_origin.len() {
        assert!(
            (point_origin[i] - point_mirrored[i]).abs() <= 1,
            "{} != {}",
            point_origin[i],
            point_mirrored[i]
        )
    }

    fn pre_order_traversal_rev<F, Meta>(node: &Node<Meta>, mut f: F)
    where
        F: FnMut(&Node<Meta>),
    {
        let mut stack: Vec<NonNull<Node<Meta>>> = vec![node.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter().rev() {
                stack.push(child.as_ref().into());
            }
        }
    }
}

fn mirror<D: Clone>(root: &Node<D>) -> Node<D> {
    let mut root = root.clone();
    root.post_order_traversal_mut(|node| {
        let n = node.children.len();
        for i in 0..n / 2 {
            node.children.swap(i, n - i - 1);
        }
    });
    root
}
