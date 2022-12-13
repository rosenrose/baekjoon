fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [x, y] = parse_int_vec(&buf)[..] {
        // let gcd = get_gcd(x, y);
        // let (x, y) = (x / gcd, y / gcd);
        // println!("{}", (x + y - 1) * gcd);

        println!("{}", x + y - get_gcd(x, y));
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

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
