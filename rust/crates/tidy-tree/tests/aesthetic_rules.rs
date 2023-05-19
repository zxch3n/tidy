use std::ptr::NonNull;

use tidy_tree::{geometry::*, Layout, Node};

pub fn assert_no_overlap_nodes(root: &Node) {
    let mut nodes: Vec<NonNull<Node>> = vec![];
    root.post_order_traversal(|node| {
        for other in nodes.iter() {
            let other = unsafe { other.as_ref() };
            if node.intersects(other) {
                let msg = format!("{} and {} overlap", node.str(), other.str());
                panic!("{}\n\n{}", msg, root.str());
            }
        }

        nodes.push(node.into());
    });
}

pub fn check_nodes_order(root: &Node) {
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

pub fn check_y_position_in_same_level(root: &Node) {
    root.pre_order_traversal(|node| {
        let mut prev = None;
        for child in node.children.iter() {
            if let Some(prev) = prev {
                assert_eq!(prev, child.y);
            }

            prev = Some(child.y);
        }
    })
}

pub fn assert_no_crossed_lines(root: &Node) {
    let mut lines: Vec<Line> = vec![];
    // println!("{}", &root.str());
    root.post_order_traversal(|node| {
        for child in node.children.iter() {
            let line = Line {
                from: Point {
                    x: node.x,
                    y: node.y + node.height,
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

pub fn assert_symmetric(root: &Node, layout: &mut dyn Layout) {
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

    assert_eq!(point_origin.len(), point_mirrored.len());
    for i in 0..point_origin.len() {
        if (point_origin[i] + point_mirrored[i]).abs() > 1e-6 {
            println!("{}", root.str());
            println!("{}", mirrored.str());
            panic!("{} != {}", point_origin[i], point_mirrored[i]);
        }
    }

    fn pre_order_traversal_rev<F>(node: &Node, mut f: F)
    where
        F: FnMut(&Node),
    {
        let mut stack: Vec<NonNull<Node>> = vec![node.into()];
        while let Some(mut node) = stack.pop() {
            let node = unsafe { node.as_mut() };
            f(node);
            for child in node.children.iter().rev() {
                stack.push(child.as_ref().into());
            }
        }
    }
}

fn mirror(root: &Node) -> Node {
    let mut root = root.clone();
    root.post_order_traversal_mut(|node| {
        node.x = 0.;
        node.y = 0.;
        node.relative_x = 0.;
        node.relative_y = 0.;
        let n = node.children.len();
        for i in 0..n / 2 {
            node.children.swap(i, n - i - 1);
        }
    });
    root
}
