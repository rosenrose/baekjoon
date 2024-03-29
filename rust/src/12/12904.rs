use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [s, t] = [(); 2].map(|_| input.next().unwrap());
    let mut t: Vec<_> = t.chars().collect();

    while t.len() > s.len() {
        if let Some('B') = t.pop() {
            t.reverse();
        }
    }

    println!("{}", (String::from_iter(t) == s) as u8);
}
