use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut max_widths = Vec::new();

    let document: Vec<Vec<_>> = buf
        .lines()
        .map(|input| {
            input
                .split(' ')
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(i, s)| {
                    if max_widths.get(i) == None {
                        max_widths.push(s.len());
                    } else {
                        max_widths[i] = max_widths[i].max(s.len());
                    }

                    s
                })
                .collect()
        })
        .collect();
    // println!("{document:?}\n{max_widths:?}");
    for row in document {
        for (i, word) in row.iter().enumerate() {
            if i > 0 {
                write!(output, " ").unwrap();
            }

            if i == row.len() - 1 {
                write!(output, "{word}").unwrap();
            } else {
                write!(output, "{word:width$}", width = max_widths[i]).unwrap();
            }
        }

        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
