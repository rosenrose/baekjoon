use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let n = input.next().unwrap();
    let nums1: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let nums2 = input.skip(1);

    for num in nums2 {
        writeln!(output, "{}", if nums1.contains(&num) { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
