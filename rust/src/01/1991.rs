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

    let n: i32 = input.next().unwrap().parse().unwrap();
    let tree: HashMap<_, _> = (0..n)
        .map(|_| {
            let [node, left, right] = [(); 3].map(|_| input.next().unwrap());

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
    for order in [Orders::Pre, Orders::In, Orders::Post] {
        let mut result = String::new();

        tree_traverse(&tree, Some("A"), order, &mut result);

        println!("{result}");
    }
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
    let visit =
        |child: Option<&str>, result: &mut String| tree_traverse(tree, child, order, result);

    match order {
        Orders::Pre => {
            result.push_str(node);
            visit(left, result);
            visit(right, result);
        }
        Orders::In => {
            visit(left, result);
            result.push_str(node);
            visit(right, result);
        }
        Orders::Post => {
            visit(left, result);
            visit(right, result);
            result.push_str(node);
        }
    }
}
