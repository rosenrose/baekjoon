use std::fmt::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let mut output = String::new();
    let mut depth = 0;
    let mut last_ch = '\0';

    for ch in buf.trim().chars() {
        let indent = " ".repeat(depth * 2);

        match ch {
            '{' => {
                writeln!(output, "{indent}{ch}")?;
                depth += 1;
            }
            '}' => {
                if last_ch != '{' {
                    writeln!(output, "")?;
                }

                depth -= 1;
                let indent = " ".repeat(depth * 2);

                write!(output, "{indent}{ch}")?;
            }
            ',' => writeln!(output, "{ch}")?,
            _ => {
                if matches!(last_ch, '{' | ',') {
                    write!(output, "{indent}")?;
                }
                write!(output, "{ch}")?;
            }
        }

        last_ch = ch;
    }

    print!("{output}");
    Ok(())
}

// print!("{}", prettify_json(buf.trim(), 2, 0));
fn prettify_json(input: &str, indent_size: usize, depth: usize) -> String {
    let mut output = String::new();
    let mut idx = 0;
    let mut count = 0;
    let mut is_item = false;

    let indent = " ".repeat(indent_size * depth);

    for (i, ch) in input.char_indices() {
        match ch {
            '{' => {
                if count == 0 {
                    idx = i;
                    writeln!(output, "{indent}{ch}").unwrap();
                }

                count += 1;
            }
            '}' => {
                count -= 1;

                if count > 0 {
                    continue;
                }

                write!(
                    output,
                    "{}",
                    prettify_json(&input[idx + 1..i], indent_size, depth + 1)
                )
                .unwrap();
                write!(output, "{indent}{ch}").unwrap();

                if i == input.len() - 1 {
                    writeln!(output, "").unwrap();
                }
            }
            ',' => {
                if count > 0 {
                    continue;
                }

                if is_item {
                    is_item = false;
                    writeln!(output, "{indent}{}", &input[idx..i + 1]).unwrap();
                } else {
                    writeln!(output, "{ch}").unwrap();
                }
            }
            _ => {
                if count > 0 {
                    continue;
                }

                if !is_item {
                    is_item = true;
                    idx = i;
                }
            }
        }
    }

    if is_item {
        writeln!(output, "{indent}{}", &input[idx..]).unwrap();
    }

    output
}
