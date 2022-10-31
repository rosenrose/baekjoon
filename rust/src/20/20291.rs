use std::collections::BTreeMap;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut ext_count = BTreeMap::new();

    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
        let ext = buf.trim().split('.').next_back().unwrap().to_string();

        ext_count.entry(ext).and_modify(|c| *c += 1).or_insert(1);
    }

    for (ext, count) in ext_count {
        writeln!(stdout, "{ext} {count}").unwrap();
    }
}
