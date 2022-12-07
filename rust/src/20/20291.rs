use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let input = buf.lines();
    let mut output = String::new();

    let ext_count = input.skip(1).fold(HashMap::new(), |mut acc, filename| {
        let ext = filename.split('.').next_back().unwrap();
        acc.entry(ext).and_modify(|c| *c += 1).or_insert(1);

        acc
    });

    let mut ext_count: Vec<_> = ext_count.iter().collect();
    ext_count.sort_unstable_by_key(|(&ext, _)| ext);

    for (ext, count) in ext_count {
        writeln!(output, "{ext} {count}").unwrap();
    }

    print!("{output}");
}
