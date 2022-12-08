use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let sum: i32 = input.split(',').map(|s| s.parse::<i32>().unwrap()).sum();

        println!("{sum}");
    }
}
