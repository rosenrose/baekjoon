fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, mut b] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut steps = 1;

    while b > a {
        if b & 1 == 0 {
            b >>= 1;
        } else if b % 10 == 1 {
            b /= 10;
        } else {
            println!("-1");
            return;
        }

        if b < a {
            println!("-1");
            return;
        }

        steps += 1;
    }

    println!("{steps}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
