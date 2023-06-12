use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [a, b, c] = [(); 3].map(|_| input.next().unwrap());
    let mut memo = vec![vec![vec![0; a.len() + 1]; b.len() + 1]; c.len() + 1];
    let mut max_len = 0;

    for (i, c) in c.char_indices() {
        for (j, b) in b.char_indices() {
            for (k, a) in a.char_indices() {
                memo[i + 1][j + 1][k + 1] = if c == b && b == a {
                    memo[i][j][k] + 1
                } else {
                    memo[i][j + 1][k + 1]
                        .max(memo[i + 1][j][k + 1])
                        .max(memo[i + 1][j + 1][k])
                };

                max_len = memo[i + 1][j + 1][k + 1].max(max_len);
            }
        }
    }

    println!("{max_len}");
}
