fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [e, s, m] = parse_int_vec(&buf)[..] {
        let mut year = s;

        loop {
            if (e % 15 == year % 15) && (m % 19 == year % 19) {
                println!("{year}");
                return;
            }

            year += 28;
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
