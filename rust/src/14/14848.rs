use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    let mut count = 0;
    let mut lcm_accum = vec![(1, -1)];

    for num in input.skip(1) {
        for i in 0..lcm_accum.len() {
            let (lcm, sign) = lcm_accum[i];
            let lcm = get_lcm(lcm, num);

            if lcm > n {
                continue;
            }

            count += (n / lcm) * -sign;
            lcm_accum.push((lcm, -sign));
        }
    }

    println!("{}", n - count);
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}
