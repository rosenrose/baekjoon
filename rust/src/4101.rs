fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if let [a, b] = parse_int_vec(&buf)[..] {
            if (a, b) == (0, 0) {
                return;
            }

            println!("{}", if a > b { "Yes" } else { "No" })
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
