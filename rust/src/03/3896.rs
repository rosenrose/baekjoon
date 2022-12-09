use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for k in input.skip(1) {
        if is_prime(k) {
            writeln!(output, "0").unwrap();
            continue;
        }

        let down = (2..k).rev().filter(|&i| is_prime(i)).next().unwrap();
        let up = (k + 1..).filter(|&i| is_prime(i)).next().unwrap();

        writeln!(output, "{}", up - down).unwrap();
    }

    print!("{output}");
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    for i in (2..).take_while(|i| i * i <= num) {
        if num % i == 0 {
            return false;
        }
    }

    true
}
