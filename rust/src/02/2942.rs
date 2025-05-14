fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r, g] = parse_int_vec(&buf)[..] else {
        return;
    };
    let gcd = get_gcd(r, g);

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

    for divisor in divisors {
        println!("{divisor} {} {}", r / divisor, g / divisor);
    }
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
