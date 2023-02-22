use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (_, k) = (input.next(), input.next().unwrap());
    let mut arr: Vec<_> = input.collect();

    println!("{}", arr.select_nth_unstable_by(k - 1, |a, b| b.cmp(a)).1);
}
