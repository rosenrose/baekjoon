fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let encrypted: String = buf
        .chars()
        .map(|ch| {
            if matches!(ch, 'a'..='z' | 'A'..='Z') {
                let offset = if matches!(ch, 'a'..='z') { b'a' } else { b'A' };

                ((ch as u8 - offset + 13) % 26 + offset) as char
            } else {
                ch
            }
        })
        .collect();

    println!("{encrypted}");
}
