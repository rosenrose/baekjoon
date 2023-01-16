use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "***") {
        println!("{}", input.chars().rev().collect::<String>());
    }
}
