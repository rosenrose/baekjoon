use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();
    let mut output = String::new();

    let mut ext_counts = HashMap::new();

    for filename in input.skip(1) {
        let ext = filename.split('.').next_back().unwrap();
        ext_counts.entry(ext).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut ext_counts = Vec::from_iter(ext_counts);
    ext_counts.sort_unstable();

    for (ext, count) in ext_counts {
        writeln!(output, "{ext} {count}").unwrap();
    }

    print!("{output}");
}
