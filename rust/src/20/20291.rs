use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();
    let mut output = String::new();

    let ext_counts = input.skip(1).fold(HashMap::new(), |mut acc, filename| {
        let ext = filename.split('.').next_back().unwrap();
        acc.entry(ext).and_modify(|c| *c += 1).or_insert(1);

        acc
    });

    let mut ext_counts = Vec::from_iter(ext_counts);
    ext_counts.sort_unstable_by_key(|&(ext, _)| ext);

    for (ext, count) in ext_counts {
        writeln!(output, "{ext} {count}").unwrap();
    }

    print!("{output}");
}
