fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    println!("{}{}", "1".repeat(n), "0".repeat(n - 1));
}
