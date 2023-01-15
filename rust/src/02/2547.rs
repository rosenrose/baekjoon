use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let students = input();
        let candies: i128 = (0..students).map(|_| input()).sum();

        println!("{}", if candies % students == 0 { "YES" } else { "NO" });
    }
}
