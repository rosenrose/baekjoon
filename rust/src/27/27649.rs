use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let mut output = String::new();
    let mut cursor = 0;

    while let Some((mut start, mut end)) = find_delims(&input[cursor..]) {
        start += cursor;
        end += cursor;
        let token = &input[cursor..start];
        let delim = &input[start..end];

        match (token.is_empty(), delim.starts_with(' ')) {
            (true, true) => write!(output, ""),
            (true, false) => write!(output, "{delim} "),
            (false, true) => write!(output, "{token} "),
            (false, false) => write!(output, "{token} {delim} "),
        }
        .unwrap();

        cursor = end;
    }

    write!(output, "{}", &input[cursor..]).unwrap();
    print!("{output}");
}

fn find_delims(input: &str) -> Option<(usize, usize)> {
    for (i, ch) in input.char_indices() {
        match ch {
            '<' | '>' | '(' | ')' => {
                return Some((i, i + 1));
            }
            '&' | '|' => {
                let next_ch = input[i + 1..].chars().nth(0)?;

                if ch == next_ch {
                    return Some((i, i + 2));
                }
            }
            ' ' => {
                let end = input[i..].find(|c| c != ' ')?;

                return Some((i, i + end));
            }
            _ => (),
        }
    }

    None
}
