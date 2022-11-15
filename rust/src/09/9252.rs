fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let s: Vec<_> = buf.trim().chars().collect();
    read_line(&mut buf);

    let t = buf.trim();

    let mut table = vec![vec![0; s.len() + 1]; t.len() + 1];
    let mut max_len = 0;

    for (i, t_char) in t.char_indices() {
        for j in 0..s.len() {
            table[i + 1][j + 1] = if t_char == s[j] {
                table[i][j] + 1
            } else {
                table[i][j + 1].max(table[i + 1][j])
            };

            max_len = table[i + 1][j + 1].max(max_len);
        }
    }

    let mut lcs = String::new();
    let (mut i, mut j) = (t.len(), s.len());

    loop {
        if table[i][j] == 0 {
            break;
        }

        if table[i][j] == table[i - 1][j] {
            i -= 1;
            continue;
        }
        if table[i][j] == table[i][j - 1] {
            j -= 1;
            continue;
        }

        assert_eq!(s[j - 1], t.chars().nth(i - 1).unwrap());
        lcs.push(s[j - 1]);
        i -= 1;
        j -= 1;
    }

    println!("{max_len}");

    if !lcs.is_empty() {
        lcs = lcs.chars().rev().collect();

        println!("{lcs}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
