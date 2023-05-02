fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = parse_int_vec(&buf);
    let mut min_lcm = i32::MAX;

    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                let lcm = get_lcm(nums[i], nums[j]);
                let lcm = get_lcm(lcm, nums[k]);

                min_lcm = lcm.min(min_lcm);
            }
        }
    }

    println!("{min_lcm}");
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a / get_gcd(a, b) * b
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
