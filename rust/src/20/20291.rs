use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut ext_count = HashMap::new();

    for filename in input.skip(1) {
        let ext = filename.split('.').next_back().unwrap();
        ext_count.entry(ext).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut ext_count: Vec<_> = ext_count.iter().collect();
    ext_count.sort_unstable_by_key(|(&ext, _)| ext);

    for (ext, count) in ext_count {
        writeln!(output, "{ext} {count}").unwrap();
    }

    print!("{output}");
}
