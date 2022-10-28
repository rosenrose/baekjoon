fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        match parse_int_vec(&buf)[..] {
            [0, 0] => return,
            [a, b] if a % b == 0 => println!("multiple"),
            [a, b] if b % a == 0 => println!("factor"),
            _ => println!("neither"),
        };
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
