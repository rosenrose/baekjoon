use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let s = input.next().unwrap();
    let count = input
        .skip(1)
        .filter(|line| [line, &line[..line.len() - 1]].concat().contains(s))
        .count();

    println!("{count}");
}
