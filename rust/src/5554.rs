fn main() {
    let mut buf = String::new();

    let seconds: i32 = (0..4)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .sum();

    println!("{}\n{}", seconds / 60, seconds % 60);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
