fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);
    let nums = parse_int_vec_lines(&mut buf, n);

    let mut gcd = (nums[0] - nums[1]).abs();

    for i in 1..nums.len() - 1 {
        gcd = get_gcd((nums[i] - nums[i + 1]).abs(), gcd);
    }

    let mut count = 0;

    for i in 0..nums.len() - 1 {
        let gap = (nums[i] - nums[i + 1]).abs();

        count += (gap / gcd) - 1;
    }

    println!("{count}");
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
