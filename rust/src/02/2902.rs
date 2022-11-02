fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let short: String = buf.trim().chars().filter(|c| c.is_uppercase()).collect();

    println!("{short}");
}
