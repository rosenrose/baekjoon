use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let Some(a) = input.next() {
        if a == 0 {
            return;
        }

        let leaves = (0..a).fold(1, |leaf, _| {
            (leaf * input.next().unwrap()) - input.next().unwrap()
        });

        println!("{leaves}");
    }
}
