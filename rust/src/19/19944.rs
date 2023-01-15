fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    match parse_int_vec(&buf)[..] {
        [_, 1 | 2] => println!("NEWBIE!"),
        [n, m] if m <= n => println!("OLDBIE!"),
        _ => println!("TLE!"),
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
