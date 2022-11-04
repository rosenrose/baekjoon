fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let censored: String = buf
        .trim()
        .chars()
        .filter(|&c| !"CAMBRIDGE".contains(c))
        .collect();

    println!("{censored}");
}
