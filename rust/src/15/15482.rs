use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [s, t] = [(); 2].map(|_| input.next().unwrap());
    let mut memo = vec![vec![0; s.chars().count() + 1]; t.chars().count() + 1];
    let mut max_len = 0;

    for (i, t_char) in t.chars().enumerate() {
        for (j, s_char) in s.chars().enumerate() {
            memo[i + 1][j + 1] = if t_char == s_char {
                memo[i][j] + 1
            } else {
                memo[i][j + 1].max(memo[i + 1][j])
            };

            max_len = memo[i + 1][j + 1].max(max_len);
        }
    }

    println!("{max_len}");
}
