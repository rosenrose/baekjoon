fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let s = buf.trim().to_string();
    read_line(&mut buf);

    let t = buf.trim();

    let mut table = vec![vec![0; s.chars().count() + 1]; t.chars().count() + 1];
    let mut max_len = 0;

    for (i, t_char) in t.chars().enumerate() {
        for (j, s_char) in s.chars().enumerate() {
            table[i + 1][j + 1] = if t_char == s_char {
                table[i][j] + 1
            } else {
                table[i][j + 1].max(table[i + 1][j])
            };

            max_len = table[i + 1][j + 1].max(max_len);
        }
    }

    println!("{max_len}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
