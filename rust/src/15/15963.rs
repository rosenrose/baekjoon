fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (n, m) = buf.trim().split_once(' ').unwrap();

    println!("{}", (n == m) as u8);
}
