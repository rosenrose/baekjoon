fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", buf.as_bytes()[0]);
}
