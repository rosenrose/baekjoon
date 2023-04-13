use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (na, _) = (input.next().unwrap() as usize, input.next());
    let mut a: HashSet<_> = input.by_ref().take(na).collect();

    for num in input {
        a.remove(&num);
    }

    let mut a = Vec::from_iter(a);
    a.sort_unstable();

    println!("{}", a.len());

    for num in a {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
