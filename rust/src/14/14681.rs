fn main() {
    let mut buf = String::new();

    match parse_int_vec_lines(&mut buf, 2)[..] {
        [x, y] if x > 0 && y > 0 => println!("1"),
        [x, y] if x < 0 && y > 0 => println!("2"),
        [x, y] if x < 0 && y < 0 => println!("3"),
        [x, y] if x > 0 && y < 0 => println!("4"),
        _ => (),
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
