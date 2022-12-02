fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let nums = parse_int_vec(&buf);

        let mut max_gcd = 0;

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                max_gcd = get_gcd(nums[i], nums[j]).max(max_gcd);
            }
        }

        println!("{max_gcd}");
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
