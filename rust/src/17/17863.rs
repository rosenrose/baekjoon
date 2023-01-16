fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", if buf.starts_with("555") { "YES" } else { "NO" });
}
