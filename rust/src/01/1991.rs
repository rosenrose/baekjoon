use std::collections::HashMap;
use std::io;

#[derive(Copy, Clone)]
enum Orders {
    Pre,
    In,
    Post,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let tree: HashMap<_, _> = (0..n)
        .map(|_| {
            let (node, left, right) = (input(), input(), input());

            (
                node,
                (
                    (left != ".").then_some(left),
                    (right != ".").then_some(right),
                ),
            )
        })
        .collect();
    // println!("{tree:?}");
    let mut result = String::new();
    tree_traverse(&tree, Some("A"), Orders::Pre, &mut result);
    println!("{result}");

    result.clear();
    tree_traverse(&tree, Some("A"), Orders::In, &mut result);
    println!("{result}");

    result.clear();
    tree_traverse(&tree, Some("A"), Orders::Post, &mut result);
    println!("{result}");
}

fn tree_traverse(
    tree: &HashMap<&str, (Option<&str>, Option<&str>)>,
    root: Option<&str>,
    order: Orders,
    result: &mut String,
) {
    let Some(node) = root else {
        return;
    };

    let (left, right) = tree[node];

    match order {
        Orders::Pre => {
            result.push_str(node);
            tree_traverse(tree, left, order, result);
            tree_traverse(tree, right, order, result);
        }
        Orders::In => {
            tree_traverse(tree, left, order, result);
            result.push_str(node);
            tree_traverse(tree, right, order, result);
        }
        Orders::Post => {
            tree_traverse(tree, left, order, result);
            tree_traverse(tree, right, order, result);
            result.push_str(node);
        }
    }
}
