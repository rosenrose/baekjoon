fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let words = buf.split_whitespace();

    println!("{}", words.count());
}
