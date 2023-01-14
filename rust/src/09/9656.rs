fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", if n % 2 == 0 { "SK" } else { "CY" });
}
