fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    println!("{}", (n * (n - 1)) / 2 * (n + 1));
}
