fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = buf.trim().to_string();
    read_line(&mut buf);

    let b = buf.trim().to_string();
    read_line(&mut buf);

    let c = buf.trim();

    let mut table = vec![vec![vec![0; a.len() + 1]; b.len() + 1]; c.len() + 1];
    let mut max_len = 0;

    for (i, c) in c.char_indices() {
        for (j, b) in b.char_indices() {
            for (k, a) in a.char_indices() {
                table[i + 1][j + 1][k + 1] = if c == b && b == a {
                    table[i][j][k] + 1
                } else {
                    table[i][j + 1][k + 1].max(table[i + 1][j][k + 1].max(table[i + 1][j + 1][k]))
                };

                max_len = table[i + 1][j + 1][k + 1].max(max_len);
            }
        }
    }

    println!("{max_len}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
