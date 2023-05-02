fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        match parse_int_vec(&buf)[..] {
            [_, 1 | 2] => "NEWBIE!",
            [n, m] if m <= n => "OLDBIE!",
            _ => "TLE!",
        }
    );
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
