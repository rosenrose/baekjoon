use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let mut problems: Vec<_> = (0..n)
        .map(|_| {
            let (title, difficulty) = (input(), input());
            (difficulty, title)
        })
        .collect();

    problems.sort();

    println!("{}", problems[0].1);
}
