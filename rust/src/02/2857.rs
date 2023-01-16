use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    if !buf.lines().any(|input| input.contains("FBI")) {
        println!("HE GOT AWAY!");
        return;
    }

    buf.lines()
        .enumerate()
        .filter_map(|(i, input)| input.contains("FBI").then_some(i + 1))
        .for_each(|i| print!("{i} "));
}
