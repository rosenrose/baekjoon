fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i64 = buf.split_whitespace().flat_map(str::parse::<i64>).sum();

    println!("{sum}");
}
