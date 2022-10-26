fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: f64 = buf.trim().parse().unwrap();

    println!("{}", (n / 5.0).ceil());
}
