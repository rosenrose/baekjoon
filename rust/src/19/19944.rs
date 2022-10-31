fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, m] = parse_int_vec(&buf)[..] {
        match m {
            1 | 2 => println!("NEWBIE!"),
            m if m <= n => println!("OLDBIE!"),
            _ => println!("TLE!"),
        };
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
