fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let score: usize = buf
        .trim()
        .replace(' ', "")
        .split(|c| c == '0')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.char_indices().map(|(i, _)| i + 1))
            }
        })
        .flatten()
        .sum();

    println!("{score}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
