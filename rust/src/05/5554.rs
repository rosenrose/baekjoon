use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let seconds: i32 = buf.lines().flat_map(str::parse::<i32>).sum();

    println!("{}\n{}", seconds / 60, seconds % 60);
}
