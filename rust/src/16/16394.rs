fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let year: i32 = buf.trim().parse().unwrap();

    println!("{}", year - 1946);
}
