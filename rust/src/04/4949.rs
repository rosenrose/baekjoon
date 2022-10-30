use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();

    'outer: loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if buf == ".\n" || buf == ".\r\n" {
            return;
        }

        let mut open_close = Vec::new();

        for c in buf.trim().chars() {
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
