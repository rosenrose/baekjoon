use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let mut tree_counts = HashMap::new();
    let mut total = 0;

    for name in buf.lines() {
        tree_counts.entry(name).and_modify(|c| *c += 1).or_insert(1);
        total += 1;
    }

    let mut tree_counts = Vec::from_iter(tree_counts);
    tree_counts.sort_unstable();

    for (tree, count) in tree_counts {
        let percentage = (count as f64 / total as f64) * 100.0;

        writeln!(output, "{tree} {percentage:.4}").unwrap();
    }

    print!("{output}");
}
