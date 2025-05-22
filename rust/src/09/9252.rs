use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(str::as_bytes);

    let [s, t] = [(); 2].map(|_| input.next().unwrap());
    let mut memo = [[0; MAX]; MAX];
    let mut max_len = 0;

    for (i, t_char) in t.iter().enumerate() {
        for (j, s_char) in s.iter().enumerate() {
            memo[i + 1][j + 1] = if t_char == s_char {
                memo[i][j] + 1
            } else {
                memo[i][j + 1].max(memo[i + 1][j])
            };

            max_len = memo[i + 1][j + 1].max(max_len);
        }
    }

    let mut lcs = Vec::with_capacity(MAX);
    let (mut i, mut j) = (t.len(), s.len());

    loop {
        if memo[i][j] == 0 {
            break;
        }

        if memo[i][j] == memo[i - 1][j] {
            i -= 1;
            continue;
        }
        if memo[i][j] == memo[i][j - 1] {
            j -= 1;
            continue;
        }
        // assert_eq!(s[j - 1], t.chars().nth(i - 1).unwrap());
        lcs.push(s[j - 1]);
        i -= 1;
        j -= 1;
    }

    lcs.reverse();

    println!("{max_len}\n{}", String::from_utf8(lcs).unwrap());
}
