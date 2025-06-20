use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [a, b, c] = [(); 3].map(|_| input.next().unwrap());

    println!("{}", if b >= c { -1 } else { (a / (c - b)) + 1 });
}
