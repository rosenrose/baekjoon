use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut output = String::new();
    let mut total = 0;

    let tree_counts = buf.lines().fold(HashMap::new(), |mut acc, name| {
        acc.entry(name).and_modify(|c| *c += 1).or_insert(1);
        total += 1;

        acc
    });

    let mut tree_counts: Vec<_> = tree_counts.iter().collect();
    tree_counts.sort_unstable_by_key(|(&name, _)| name);

    for (tree, &count) in tree_counts {
        let percentage = (count as f64 / total as f64) * 100.0;

        writeln!(output, "{tree} {percentage:.4}").unwrap();
    }

    print!("{output}");
}
