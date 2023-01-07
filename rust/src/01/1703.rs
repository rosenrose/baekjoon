use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
