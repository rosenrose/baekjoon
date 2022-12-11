use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let extra_apple = (0..n).fold(0, |acc, _| {
        let (student, apple) = (input.next().unwrap(), input.next().unwrap());
        acc + (apple % student)
    });

    println!("{extra_apple}");
}
