fn main() {
    let mut buf = String::new();

    let sum: i32 = (0..5)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .sum();

    println!("{sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
