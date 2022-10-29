fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "#" {
            return;
        }

        let vowels = buf
            .trim()
            .chars()
            .filter(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            })
            .count();

        println!("{vowels}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
