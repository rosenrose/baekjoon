fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, k] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);

        let nums = parse_int_vec(&buf);
        let mut count = 0;

        for bit in 1..1 << k {
            let indices: Vec<_> = (0..k).filter(|index| bit & (1 << index) != 0).collect();
            // println!("{indices:?}");
            let mut lcm = 1;

            for num in indices.iter().map(|&i| nums[i as usize]) {
                lcm = get_lcm(lcm, num);

                if lcm > n {
                    break;
                }
            }

            let multiple_count = n / lcm;

            count += if indices.len() % 2 == 1 {
                multiple_count
            } else {
                -multiple_count
            };
        }

        println!("{count}");
    }
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
