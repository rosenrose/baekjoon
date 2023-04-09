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
    tree_traverse(&tree, Some("A"), Orders::Pre);
    println!("");
    tree_traverse(&tree, Some("A"), Orders::In);
    println!("");
    tree_traverse(&tree, Some("A"), Orders::Post);
}

fn tree_traverse(
    tree: &HashMap<&str, (Option<&str>, Option<&str>)>,
    root: Option<&str>,
    order: Orders,
) {
    let Some(node) = root else {
        return;
    };

    let visit = |child: Option<&str>| tree_traverse(tree, child, order);
    let (left, right) = tree[node];

    match order {
        Orders::Pre => {
            print!("{node}");
            visit(left);
            visit(right);
        }
        Orders::In => {
            visit(left);
            print!("{node}");
            visit(right);
        }
        Orders::Post => {
            visit(left);
            visit(right);
            print!("{node}");
        }
    }
}
