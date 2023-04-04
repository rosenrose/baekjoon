use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let mut output = String::new();
    let mut cursor = 0;

    while let Some((start, end)) = find_delims(input, cursor) {
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

    write!(output, "{}", buf[cursor..].trim()).unwrap();
    print!("{output}");
}

fn find_delims(input: &str, start: usize) -> Option<(usize, usize)> {
    let input = &input[start..];
    let mut i = 0;

    while i < input.len() {
        let ch = &input[i..i + 1];

        match ch {
            "<" | ">" | "(" | ")" => {
                return Some((start + i, start + i + 1));
            }
            "&" | "|" => {
                if i + 1 == input.len() {
                    return None;
                }

                let next_ch = &input[i + 1..i + 2];

                if ch == next_ch {
                    return Some((start + i, start + i + 2));
                }
            }
            " " => {
                let end = input[i..].find(|c| c != ' ')?;
                return Some((start + i, start + i + end));
            }
            _ => (),
        }

        i += 1;
    }

    None
}
