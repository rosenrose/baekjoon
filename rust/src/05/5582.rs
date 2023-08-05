use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(str::as_bytes);
    let [s, t] = [(); 2].map(|_| input.next().unwrap());

    let mut memo = vec![vec![0; s.len() + 1]; t.len() + 1];
    let mut max_len = 0;

    for (i, t_char) in t.iter().enumerate() {
        for (j, s_char) in s.iter().enumerate() {
            if t_char != s_char {
                continue;
            }

            memo[i + 1][j + 1] = memo[i][j] + 1;
            max_len = memo[i + 1][j + 1].max(max_len);
        }
    }

    println!("{max_len}");
}
