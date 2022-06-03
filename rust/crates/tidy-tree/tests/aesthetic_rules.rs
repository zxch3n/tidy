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

pub fn check_nodes_order<D>(root: &Node<D>) {
    root.pre_order_traversal(|node| {
        let mut prev = None;
        for child in node.children.iter() {
            if let Some(prev) = prev {
                assert!(prev < child.x);
            }

            prev = Some(child.x);
        }
    })
}

pub fn check_y_position_in_same_level<D>(root: &Node<D>) {
    root.pre_order_traversal(|node| {
        let mut prev = None;
        for child in node.children.iter() {
            if let Some(prev) = prev {
                assert!(prev == child.y);
            }

            prev = Some(child.y);
        }
    })
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

pub fn assert_parent_visually_centered<D: Debug>(root: &Node<D>) {
    root.pre_order_traversal(|node| {
        let n = node.children.len();
        if n == 0 {
            return;
        }

        let middle = if n % 2 == 0 {
            let m = n / 2;
            let a = &node.children[m - 1];
            let b = &node.children[m];
            (a.x + b.x) / 2.
        } else {
            node.children[n / 2].x
        };
        assert!(
            (node.x - middle).abs() < 1e-6,
            "parent node is not centered {} {}",
            node.x,
            middle
        );
    });
}

pub fn assert_symmetric<D: Debug + Clone>(root: &Node<D>, layout: &mut dyn Layout<Meta = D>) {
    let mut mirrored = mirror(root);
    layout.layout(&mut mirrored);
    let mut point_origin: Vec<Coord> = vec![];
    let mut point_mirrored: Vec<Coord> = vec![];
    root.pre_order_traversal(|node| {
        point_origin.push(node.x);
    });
    pre_order_traversal_rev(&mirrored, |node| {
        point_mirrored.push(node.x);
    });
    // println!("{:#?}", root);
    // println!("{:#?}", mirrored);

    assert_eq!(point_origin.len(), point_mirrored.len());
    for i in 0..point_origin.len() {
        assert!(
            (point_origin[i] + point_mirrored[i]).abs() <= 1e-6,
            "{} != {}",
            point_origin[i],
            -point_mirrored[i]
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

        let node_ptr = node.into();
        for child in node.children.iter_mut() {
            child.parent = Some(node_ptr);
        }
    });
    root
}
