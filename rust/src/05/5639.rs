use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let pre_order: Vec<_> = buf.lines().flat_map(str::parse::<i32>).collect();
    visit_post_order(&pre_order, &mut output);

    print!("{output}");
}

fn visit_post_order(pre_order: &[i32], output: &mut String) {
    let Some(&root) = pre_order.get(0) else {
        return;
    };

    let right_start = pre_order
        .iter()
        .position(|&node| node > root)
        .unwrap_or(pre_order.len());

    visit_post_order(&pre_order[1..right_start], output);
    visit_post_order(&pre_order[right_start..], output);

    writeln!(output, "{root}").unwrap();
}
