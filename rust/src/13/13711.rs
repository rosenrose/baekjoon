use std::collections::HashMap;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    let a: HashMap<_, _> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| (parse_int(s), i))
        .collect();

    let mut index_arr = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| a.get(&parse_int(s)).unwrap());

    let mut lis = vec![index_arr.next().unwrap()];

    for index in index_arr {
        if index > *lis.last().unwrap() {
            lis.push(index);
            continue;
        }

        let pos = match lis.binary_search(&index) {
            Ok(i) => i,
            Err(i) => i,
        };

        lis[pos] = index;
    }

    writeln!(stdout, "{}", lis.len()).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
