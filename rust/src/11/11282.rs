fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: u32 = buf.trim().parse().unwrap();

    println!("{}", char::from_u32('가' as u32 + n - 1).unwrap());
}
