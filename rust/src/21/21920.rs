fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i64>().unwrap();

    let nums = buf.to_string();
    let nums = nums.split_whitespace().map(parse_int);
    read_line(&mut buf);

    let x: i64 = parse_int(buf.trim());
    let (mut sum, mut count) = (0, 0);

    for num in nums {
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
