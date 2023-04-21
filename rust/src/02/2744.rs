fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let reversed: String = buf
        .trim()
        .chars()
        .flat_map(|ch| {
            if ch.is_lowercase() {
                ch.to_uppercase().next()
            } else {
                ch.to_lowercase().next()
            }
        })
        .collect();

    println!("{reversed}");
}
