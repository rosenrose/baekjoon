fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    for _ in 1..=n {
        println!("{}", "*".repeat(n));
    }
}
