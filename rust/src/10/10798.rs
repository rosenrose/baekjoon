fn main() {
    let mut buf = String::new();

    let words = parse_str_vec_lines(&mut buf, 5);
    let longest = words.iter().map(|word| word.len()).max().unwrap();

    for i in 0..longest {
        for word in &words {
            if let Some(c) = word.chars().nth(i) {
                print!("{c}");
            }
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
