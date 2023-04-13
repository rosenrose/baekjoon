use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (a, _) = (input.next().unwrap() as usize, input.next());
    let mut diff: HashSet<_> = input.by_ref().take(a).collect();

    for num in input {
        if diff.contains(&num) {
            diff.remove(&num);
        } else {
            diff.insert(num);
        }
    }

    println!("{}", diff.len());
}
