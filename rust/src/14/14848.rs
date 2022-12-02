use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.split_whitespace().next().unwrap());
    read_line(&mut buf);

    let arr = parse_int_set(&buf);
    let mut count = 0;
    let mut lcm_accum = vec![(1, -1)];

    for num in arr {
        for i in 0..lcm_accum.len() {
            let (lcm, sign) = lcm_accum[i];
            let lcm = get_lcm(lcm, num);

            if lcm > n {
                continue;
            }

            count += (n / lcm) * -sign;
            lcm_accum.push((lcm, -sign));
        }
    }

    println!("{}", n - count);
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

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_set(buf: &String) -> HashSet<i64> {
    buf.split_whitespace().map(parse_int).collect()
}
