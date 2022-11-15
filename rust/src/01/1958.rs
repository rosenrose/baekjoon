fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a: Vec<_> = buf.trim().chars().collect();
    read_line(&mut buf);

    let b: Vec<_> = buf.trim().chars().collect();
    read_line(&mut buf);

    let c = buf.trim();

    let mut table = vec![vec![vec![0; a.len() + 1]; b.len() + 1]; c.len() + 1];
    let mut max_len = 0;

    for (i, c_char) in c.char_indices() {
        for j in 0..b.len() {
            for k in 0..a.len() {
                table[i + 1][j + 1][k + 1] = if c_char == b[j] && b[j] == a[k] {
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
