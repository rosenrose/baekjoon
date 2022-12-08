use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    if !buf.lines().any(|input| input.contains("FBI")) {
        println!("HE GOT AWAY!");
        return;
    }

    buf.lines()
        .enumerate()
        .filter_map(|(i, input)| input.contains("FBI").then(|| i + 1))
        .for_each(|i| print!("{i} "));
}
