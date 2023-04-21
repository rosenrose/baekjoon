fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let censored: String = buf.trim().matches(|ch| !"CAMBRIDGE".contains(ch)).collect();

    println!("{censored}");
}
