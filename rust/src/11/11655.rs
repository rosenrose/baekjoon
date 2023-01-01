fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let encrypted: String = buf
        .chars()
        .map(|c| {
            if matches!(c, 'a'..='z' | 'A'..='Z') {
                let offset = if matches!(c, 'a'..='z') { 'a' } else { 'A' } as u8;

                ((c as u8 - offset + 13) % 26 + offset) as char
            } else {
                c
            }
        })
        .collect();

    println!("{encrypted}");
}
