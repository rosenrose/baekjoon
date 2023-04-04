fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let split_by_char = |ch: char| buf.trim().split(ch).filter(|s| !s.is_empty());
    let split_by_0 = split_by_char('0');
    let split_by_1 = split_by_char('1');

    println!("{}", split_by_0.count().min(split_by_1.count()));
}
