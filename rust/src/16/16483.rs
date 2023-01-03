fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let t: f64 = buf.trim().parse().unwrap();

    println!("{:.0}", t * t / 4.0);
}
