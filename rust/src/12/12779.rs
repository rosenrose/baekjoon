fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        let range = b - a;
        let square_nums_count = sqrt_binary_search(b, 0, b) - sqrt_binary_search(a, 0, a);

        if square_nums_count == 0 {
            println!("0");
            return;
        }

        let gcd = get_gcd(square_nums_count, range);

        println!("{}/{}", square_nums_count / gcd, range / gcd);
    }
}

// (n as f64).sqrt() as i64 -> 정확도 손실됨
fn sqrt_binary_search(n: i64, left: i64, right: i64) -> i64 {
    if (right as i128 * right as i128) <= n as i128 {
        return right;
    }

    if right - left <= 1 {
        return left;
    }

    let mid = (left as i128 + right as i128) / 2;

    if mid * mid <= n as i128 {
        sqrt_binary_search(n, mid as i64, right)
    } else {
        sqrt_binary_search(n, left, mid as i64)
    }
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
