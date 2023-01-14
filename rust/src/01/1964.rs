fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let dots = |n: i64| (n * (3 * n + 5)) / 2 + 1;

    println!("{}", dots(n) % 45678);
}
