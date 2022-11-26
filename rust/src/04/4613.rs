fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "#" {
            return;
        }

        let checksum: usize = buf
            .trim()
            .char_indices()
            .map(|(i, c)| match c {
                'A'..='Z' => (i + 1) * (c as u8 - 'A' as u8 + 1) as usize,
                _ => 0,
            })
            .sum();

        println!("{checksum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
