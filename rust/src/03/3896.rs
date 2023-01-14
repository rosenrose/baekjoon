use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for k in input.skip(1) {
        if is_prime(k) {
            writeln!(output, "0").unwrap();
            continue;
        }

        let down = (2..k).rev().find(|&i| is_prime(i)).unwrap();
        let up = (k + 1..).find(|&i| is_prime(i)).unwrap();

        writeln!(output, "{}", up - down).unwrap();
    }

    print!("{output}");
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
