use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let pre_order: Vec<_> = buf.lines().flat_map(str::parse::<i32>).collect();
    let post_order = build_post_order(&pre_order, 0, pre_order.len() - 1);

    for node in post_order {
        writeln!(output, "{node}").unwrap();
    }

    print!("{output}");
}

fn build_post_order(pre_order: &[i32], start: usize, end: usize) -> Vec<i32> {
    if start > end {
        return Vec::new();
    }

    let root = pre_order[start];
    let mut post_order = Vec::new();
    let mut right_tree_start = start + 1;

    while right_tree_start <= end && pre_order[right_tree_start] < root {
        right_tree_start += 1;
    }

    post_order.extend(build_post_order(pre_order, start + 1, right_tree_start - 1));
    post_order.extend(build_post_order(pre_order, right_tree_start, end));
    post_order.push(root);

    post_order
}
