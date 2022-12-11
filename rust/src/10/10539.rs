use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut sum = 0;

    buf.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, avg)| {
            let num = avg * (i as i32 + 1) - sum;
            print!("{num} ");

            sum += num;
        });
}
