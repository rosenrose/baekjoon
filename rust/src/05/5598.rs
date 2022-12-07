fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let offset = 'A' as u8;
    let decrypted: String = buf
        .trim()
        .as_bytes()
        .iter()
        .map(|c| ((c - offset + 23) % 26 + offset) as char)
        .collect();

    println!("{decrypted}");
}
