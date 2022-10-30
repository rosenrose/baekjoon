fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let code = buf.chars().nth(0).unwrap() as u32;

    println!("{}", code - '가' as u32 + 1);
}
