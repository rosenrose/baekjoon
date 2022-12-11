use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let count = buf
        .lines()
        .skip(1)
        .filter(|input| {
            input
                .split('-')
                .next_back()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                <= 90
        })
        .count();

    println!("{count}");
}
