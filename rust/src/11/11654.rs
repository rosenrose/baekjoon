fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", buf.chars().nth(0).unwrap() as u8);
}
