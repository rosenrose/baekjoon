use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (index, max) = buf
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .max_by_key(|&(_, n)| n)
        .unwrap();
    let index = index + 1;

    println!("{max}\n{index}");
}
