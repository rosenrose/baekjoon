fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i64 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let nums = &parse_int_vec(&buf)[1..];

        let mut sum = 0;

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                sum += get_gcd(nums[i], nums[j]);
            }
        }

        println!("{sum}");
    }
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
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

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
