fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let reversed: String = buf
        .trim()
        .chars()
        .map(|c| {
            (if c.is_lowercase() {
                c.to_uppercase().next()
            } else {
                c.to_lowercase().next()
            })
            .unwrap()
        })
        .collect();

    println!("{reversed}");
}
