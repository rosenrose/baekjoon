use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    input.next();

    let mut diff: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();

    for num in input {
        if diff.contains(&num) {
            diff.remove(&num);
        } else {
            diff.insert(num);
        }
    }

    println!("{}", diff.len());
}
