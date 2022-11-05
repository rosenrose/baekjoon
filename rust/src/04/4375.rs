use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let n: i32 = line.parse().unwrap();

        let (mut remainder, mut exp_remainder) = (1 % n, 1 % n);
        let mut count = 1;

        while remainder != 0 {
            exp_remainder = (exp_remainder % n * 10 % n) % n;
            remainder = (exp_remainder % n + remainder % n) % n;

            count += 1;
        }

        println!("{count}");
    }
}
