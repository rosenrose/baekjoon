fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: f64 = buf.trim().parse().unwrap();

    println!("{:.10E}", 3.0_f64.sqrt() * n * n / 4.0);
}
