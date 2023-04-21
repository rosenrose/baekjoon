fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let short: String = buf.trim().matches(char::is_uppercase).collect();

    println!("{short}");
}
