use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = buf
        .lines()
        .take_while(|&input| input != "-1")
        .flat_map(str::parse::<i32>)
        .sum();

    println!("{sum}");
}
