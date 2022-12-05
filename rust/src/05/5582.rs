use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    let (s, t) = (input.next().unwrap(), input.next().unwrap());

    let mut table = vec![vec![0; s.len() + 1]; t.len() + 1];
    let mut max_len = 0;
    // let (mut x, mut y) = (0, 0);

    for (i, t_char) in t.char_indices() {
        for (j, s_char) in s.char_indices() {
            if t_char != s_char {
                continue;
            }

            table[i + 1][j + 1] = table[i][j] + 1;
            max_len = table[i + 1][j + 1].max(max_len);
            // if table[i + 1][j + 1] > max_len {
            //     max_len = table[i + 1][j + 1];
            //     (x, y) = (j, i);
            // }
        }
    }

    // let mut lc_string = String::new();

    // for offset in 0..max_len {
    //     assert_eq!(s[x - offset], t.chars().nth(y - offset).unwrap());
    //     lc_string.push(s[x - offset]);
    // }

    // lc_string = lc_string.chars().rev().collect();
    // println!("{lc_string}");

    println!("{max_len}");
}
