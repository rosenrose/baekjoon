use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let (a, b, c) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let mut table = vec![vec![vec![0; a.len() + 1]; b.len() + 1]; c.len() + 1];
    let mut max_len = 0;

    for (i, c) in c.char_indices() {
        for (j, b) in b.char_indices() {
            for (k, a) in a.char_indices() {
                table[i + 1][j + 1][k + 1] = if c == b && b == a {
                    table[i][j][k] + 1
                } else {
                    table[i][j + 1][k + 1]
                        .max(table[i + 1][j][k + 1])
                        .max(table[i + 1][j + 1][k])
                };

                max_len = table[i + 1][j + 1][k + 1].max(max_len);
            }
        }
    }

    println!("{max_len}");
}
