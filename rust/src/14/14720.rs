use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let (_, count) = input.fold((-1, 0), |(milk, acc), next| match (milk, next) {
        (_, 0) if acc == 0 => (next, 1),
        (0, 1) | (1, 2) | (2, 0) => (next, acc + 1),
        _ => (milk, acc),
    });

    println!("{count}");
}
