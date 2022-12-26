fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    println!("{}\n{}", n * (n - 1) * (n - 2) / 6, 3);
}
