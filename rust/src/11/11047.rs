fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, mut k] = parse_int_vec(&buf)[..] {
        let coins = parse_int_vec_lines(&mut buf, n);
        let mut count = 0;

        for coin in coins.iter().rev() {
            count += k / coin;
            k %= coin;
        }

        println!("{count}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
