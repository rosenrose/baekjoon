use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut infos: Vec<_> = (1..=input())
        .map(|i| (i, (input(), input(), input())))
        .collect();

    let (first, _) = infos
        .select_nth_unstable_by_key(0, |&(_, (s, c, t))| (std::cmp::Reverse(s), c, t))
        .1;

    println!("{first}");
}
