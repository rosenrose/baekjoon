fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let sum: i32 = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum();

    println!("{sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
