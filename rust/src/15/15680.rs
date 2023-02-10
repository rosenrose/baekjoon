fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        if let Ok(0) = buf.trim().parse() {
            "YONSEI"
        } else {
            "Leading the Way to the Future"
        }
    );
}
