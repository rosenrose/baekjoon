use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let mut dancing = HashSet::from(["ChongChong"]);

    for (a, b) in (0..n).map(|_| (input(), input())) {
        if dancing.contains(a) {
            dancing.insert(b);
        }
        if dancing.contains(b) {
            dancing.insert(a);
        }
    }

    println!("{}", dancing.len());
}
