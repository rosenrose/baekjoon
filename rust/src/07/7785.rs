use std::cmp::Reverse;
use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: usize = input().parse().unwrap();
    let mut names = HashSet::with_capacity(n);

    for (name, in_out) in (0..n).map(|_| (input(), input())) {
        match in_out {
            "enter" => {
                names.insert(name);
            }
            "leave" => {
                names.remove(name);
            }
            _ => unreachable!(),
        }
    }

    let mut names = Vec::from_iter(names);
    names.sort_unstable_by_key(|&name| Reverse(name));

    print!("{}", names.join("\n"));
}
