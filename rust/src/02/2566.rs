use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut max = -1;
    let (mut x, mut y) = (0, 0);

    for i in 0..9 {
        for j in 0..9 {
            let num = input.next().unwrap();

            if num > max {
                (max, x, y) = (num, i + 1, j + 1);
            }
        }
    }

    println!("{max}\n{x} {y}");
}
