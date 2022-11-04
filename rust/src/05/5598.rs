fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let decrypted: String = buf
        .trim()
        .chars()
        .map(|c| {
            let offset = 'A' as u8;

            ((c as u8 - offset + 23) % 26 + offset) as char
        })
        .collect();

    println!("{decrypted}");
}
