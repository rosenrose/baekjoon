use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = buf.lines().map(|input| input.parse::<i32>().unwrap()).sum();

    println!("{sum}");
}
