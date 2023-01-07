use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let x = input.next_back().unwrap();
    let (mut sum, mut count) = (0, 0);

    for num in input.skip(1) {
        if get_gcd(num, x) == 1 {
            sum += num;
            count += 1;
        }
    }

    println!("{:.10}", sum as f64 / count as f64);
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
