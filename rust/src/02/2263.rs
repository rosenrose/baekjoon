use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let in_order: Vec<_> = input.by_ref().take(n).collect();
    let post_order: Vec<_> = input.collect();

    visit_pre_order(&in_order, &post_order, &mut output);
    print!("{output}");
}

fn visit_pre_order(in_order: &[i32], post_order: &[i32], output: &mut String) {
    let Some(&root) = post_order.last() else {
        return;
    };

    let left_tree_len = in_order.iter().position(|&node| node == root).unwrap();

    write!(output, "{root} ").unwrap();
    visit_pre_order(
        &in_order[..left_tree_len],
        &post_order[..left_tree_len],
        output,
    );
    visit_pre_order(
        &in_order[left_tree_len + 1..],
        &post_order[left_tree_len..post_order.len() - 1],
        output,
    );
}
