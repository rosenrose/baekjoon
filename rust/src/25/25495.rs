use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (consume_accum, _, _) = input
        .skip(1)
        .fold((0, 2, 0), |(acc, mut consume, phone), next| {
            if next == phone {
                consume *= 2;
            } else {
                consume = 2;
            }

            if acc + consume >= 100 {
                (0, 2, 0)
            } else {
                (acc + consume, consume, next)
            }
        });

    println!("{consume_accum}");
}
