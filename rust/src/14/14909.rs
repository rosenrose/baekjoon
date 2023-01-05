fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        buf.split_ascii_whitespace()
            .filter(|&s| s != "0" && !s.starts_with('-'))
            .count()
    );
}
