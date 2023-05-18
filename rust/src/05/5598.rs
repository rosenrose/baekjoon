fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let offset = b'A';
    let decrypted: String = buf
        .trim()
        .as_bytes()
        .iter()
        .map(|ch| ((ch - offset + 23) % 26 + offset) as char)
        .collect();

    println!("{decrypted}");
}
