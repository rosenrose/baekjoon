fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let y: i32 = buf.trim().parse().unwrap();

    println!("{}", y - 543);
}
