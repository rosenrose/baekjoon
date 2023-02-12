use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    const NO: &str = "no";

    'outer: for input in buf.lines().take_while(|&input| input != ".") {
        let mut parens = Vec::new();

        for ch in input.chars() {
            match ch {
                '(' | '[' => parens.push(ch),
                ')' | ']' => {
                    let Some(last) = parens.pop() else {
                        writeln!(output, "{NO}").unwrap();
                        continue 'outer;
                    };

                    if !matches!((last, ch), ('(', ')') | ('[', ']')) {
                        writeln!(output, "{NO}").unwrap();
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        writeln!(output, "{}", if parens.is_empty() { "yes" } else { NO }).unwrap();
    }

    print!("{output}")
}
