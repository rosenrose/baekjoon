fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let mbti = buf.trim().to_string();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    let count = (0..n)
        .map(|_| {
            read_line(&mut buf);
            buf.trim().to_string()
        })
        .filter(|s| s == &mbti)
        .count();

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
