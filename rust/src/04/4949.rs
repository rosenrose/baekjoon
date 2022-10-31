use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    'outer: for line in buf.lines() {
        if line == "." {
            return;
        }

        let mut open_close = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' => open_close.push(c),
                ')' | ']' => match open_close.pop() {
                    Some(letter) => {
                        if (c == ')' && letter != '(') || (c == ']' && letter != '[') {
                            writeln!(stdout, "no").unwrap();
                            continue 'outer;
                        }
                    }
                    None => {
                        writeln!(stdout, "no").unwrap();
                        continue 'outer;
                    }
                },
                _ => (),
            };
        }

        writeln!(
            stdout,
            "{}",
            if open_close.is_empty() { "yes" } else { "no" }
        )
        .unwrap();
    }
}
