use std::collections::BTreeMap;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut trees = BTreeMap::new();
    let mut total = 0;

    buf.lines().for_each(|line| {
        trees.entry(line).and_modify(|c| *c += 1).or_insert(1);
        total += 1;
    });

    for (tree, count) in trees {
        let percentage = (count as f64 / total as f64) * 100.0;
        let rounded = (percentage * 10000.0).round() / 10000.0;

        writeln!(stdout, "{tree} {rounded:.4}").unwrap();
    }
}
