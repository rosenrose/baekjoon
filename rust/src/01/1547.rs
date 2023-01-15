use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let ball = (0..input()).fold(1, |ball, _| match (ball, (input(), input())) {
        (1, (1, 3) | (3, 1)) | (2, (2, 3) | (3, 2)) => 3,
        (1, (1, 2) | (2, 1)) | (3, (2, 3) | (3, 2)) => 2,
        (2, (1, 2) | (2, 1)) | (3, (1, 3) | (3, 1)) => 1,
        _ => ball,
    });

    println!("{ball}");
}
