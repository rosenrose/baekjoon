use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let mut depth = 0;
    let mut last_ch = '\0';

    for ch in buf.trim().chars() {
        let indent = " ".repeat(depth * 2);

        match ch {
            '{' => {
                writeln!(output, "{indent}{ch}").unwrap();
                depth += 1;
            }
            '}' => {
                if last_ch != '{' {
                    writeln!(output, "").unwrap();
                }

                depth -= 1;
                let indent = " ".repeat(depth * 2);

                write!(output, "{indent}{ch}").unwrap();
            }
            ',' => writeln!(output, "{ch}").unwrap(),
            _ => {
                if matches!(last_ch, '{' | ',') {
                    write!(output, "{indent}").unwrap();
                }
                write!(output, "{ch}").unwrap();
            }
        }

        last_ch = ch;
    }

    print!("{output}");
}
