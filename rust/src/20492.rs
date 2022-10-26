fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: f64 = buf.trim().parse().unwrap();

    println!("{} {}", n * (1.0 - 0.22), n * (1.0 - 0.2 * 0.22));
}
