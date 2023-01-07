use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut ball = 1;

    for _ in 0..input.next().unwrap() {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());

        ball = match (ball, (x, y)) {
            (1, (1, 3) | (3, 1)) | (2, (2, 3) | (3, 2)) => 3,
            (1, (1, 2) | (2, 1)) | (3, (2, 3) | (3, 2)) => 2,
            (2, (1, 2) | (2, 1)) | (3, (1, 3) | (3, 1)) => 1,
            _ => continue,
        };
    }

    println!("{ball}");
}
