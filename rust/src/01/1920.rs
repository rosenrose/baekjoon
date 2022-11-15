use std::collections::HashSet;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    let nums1 = parse_int_set(lines.next().unwrap());
    lines.next();

    let nums2 = lines.next().unwrap().split_whitespace().map(parse_int);

    for num in nums2 {
        writeln!(stdout, "{}", if nums1.contains(&num) { 1 } else { 0 }).unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_set(buf: &str) -> HashSet<i32> {
    buf.split_whitespace().map(parse_int).collect()
}
