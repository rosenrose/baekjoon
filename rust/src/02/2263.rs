use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut in_order_indices = vec![0; n + 1];
    let mut in_order = Vec::with_capacity(n);

    for node in input.by_ref().take(n) {
        in_order_indices[node as usize] = in_order.len();
        in_order.push(node);
    }

    let post_order: Vec<_> = input.collect();

    visit_pre_order(
        (&in_order, (0, n)),
        (&post_order, (0, n)),
        &in_order_indices,
        &mut output,
    );
    print!("{output}");
}

fn visit_pre_order(
    (in_order, in_order_range): (&[i32], (usize, usize)),
    (post_order, post_order_range): (&[i32], (usize, usize)),
    in_order_indices: &[usize],
    output: &mut String,
) {
    if in_order_range.0 == in_order_range.1 {
        return;
    }

    let root = post_order[post_order_range.1 - 1];
    let root_idx = in_order_indices[root as usize];
    let left_tree_len = root_idx - in_order_range.0;

    write!(output, "{root} ").unwrap();
    visit_pre_order(
        (in_order, (in_order_range.0, root_idx)),
        (
            post_order,
            (post_order_range.0, post_order_range.0 + left_tree_len),
        ),
        in_order_indices,
        output,
    );
    visit_pre_order(
        (in_order, (root_idx + 1, in_order_range.1)),
        (
            post_order,
            (post_order_range.0 + left_tree_len, post_order_range.1 - 1),
        ),
        in_order_indices,
        output,
    );
}
