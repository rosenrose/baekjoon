use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let s: String = input
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| c.is_alphabetic().then(|| c))
        .collect();
    let k = input.next().unwrap();

    println!("{}", if s.contains(k) { 1 } else { 0 });
}
