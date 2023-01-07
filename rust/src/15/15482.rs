use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (s, t) = (input.next().unwrap(), input.next().unwrap());
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
