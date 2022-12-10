use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    let (s, t) = (input.next().unwrap(), input.next().unwrap());

    let mut table = vec![vec![0; s.len() + 1]; t.len() + 1];
    let mut max_len = 0;

    for (i, t_char) in t.char_indices() {
        for (j, s_char) in s.char_indices() {
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
