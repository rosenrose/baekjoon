use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (index, max) = buf
        .lines()
        .flat_map(str::parse::<i32>)
        .enumerate()
        .max_by_key(|&(_, n)| n)
        .unwrap();
    let index = index + 1;

    println!("{max}\n{index}");
}
