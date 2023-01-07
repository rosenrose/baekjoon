use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = buf
        .lines()
        .map(|s| (s.parse::<i32>().unwrap()).max(40))
        .sum();

    println!("{}", sum / 5);
}
