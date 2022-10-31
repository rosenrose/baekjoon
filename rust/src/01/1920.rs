use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let nums1 = parse_int_set(&buf);
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let nums2 = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    for num in nums2 {
        let has_num = match nums1.get(&num) {
            Some(_) => 1,
            None => 0,
        };

        writeln!(stdout, "{has_num}").unwrap();
    }
}

fn parse_int_set(buf: &String) -> HashSet<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
