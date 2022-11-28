fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    println!("{}", binary_search(n, 0, (n as f64).sqrt() as i64 + 1));
}

fn binary_search(value: i64, left: i64, right: i64) -> i64 {
    if left * left >= value {
        return left;
    }

    if right - left == 1 {
        return right;
    }

    let mid = (right + left) / 2;

    if mid * mid >= value {
        binary_search(value, left, mid)
    } else {
        binary_search(value, mid, right)
    }
}
