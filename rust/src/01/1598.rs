use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap() - 1);
    let vertical = (a % 4).abs_diff(b % 4);
    let horizontal = (a / 4).abs_diff(b / 4);

    println!("{}", vertical + horizontal);
}
