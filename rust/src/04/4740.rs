use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().take_while(|&input| input != "***") {
        println!("{}", input.chars().rev().collect::<String>());
    }
}
