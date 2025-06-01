use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap());

    match a.cmp(&b) {
        Ordering::Less => println!("<"),
        Ordering::Equal => println!("=="),
        Ordering::Greater => println!(">"),
    }
}
