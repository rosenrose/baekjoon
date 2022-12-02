use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let mut tokens = buf.split_whitespace().map(parse_int);
    let (right, left) = (tokens.next_back().unwrap(), tokens.next_back().unwrap());
    read_line(&mut buf);

    let nums = parse_int_set(&buf);
    let mut count = 0;
    let mut lcm_accum = vec![(1, -1)];

    for num in nums {
        for i in 0..lcm_accum.len() {
            let (lcm, sign) = lcm_accum[i];
            let lcm = get_lcm(lcm, num);

            if lcm > right {
                continue;
            }

            count += ((right / lcm) - ((left - 1) / lcm)) * -sign;
            lcm_accum.push((lcm, -sign));
        }
    }

    println!("{count}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
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

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_set(buf: &String) -> HashSet<i64> {
    buf.split_whitespace().map(parse_int).collect()
}
