use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    while let Some(n) = input.next() {
        if n == 0 {
            break;
        }

        let mut gcd_set = HashSet::<i32>::new();
        let mut gcd_accum = HashSet::new();

        for _ in 0..n {
            let num = input.next().unwrap();

            gcd_accum = gcd_accum.iter().map(|&g| gcd(g, num)).collect();
            gcd_accum.insert(num);

            gcd_set.extend(&gcd_accum);
        }

        writeln!(output, "{}", gcd_set.len()).unwrap();
    }

    print!("{output}");
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}
