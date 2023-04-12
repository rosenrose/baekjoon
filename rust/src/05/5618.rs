use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let gcd = get_gcd(input.skip(1));
    let mut divisors = Vec::new();

    for i in (1..).take_while(|i| i * i <= gcd) {
        if gcd % i != 0 {
            continue;
        }

        divisors.push(i);

        if i != gcd / i {
            divisors.push(gcd / i);
        }
    }

    divisors.sort();

    for divisor in divisors {
        println!("{divisor}");
    }
}

fn get_gcd(nums: impl Iterator<Item = i32>) -> i32 {
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
