use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (s, t) = (input.next().unwrap(), input.next().unwrap());
    let mut t: Vec<_> = t.chars().collect();

    while t.len() > s.len() {
        if let Some('B') = t.pop() {
            t.reverse();
        }
    }

    println!("{}", if String::from_iter(t) == s { 1 } else { 0 });
}
