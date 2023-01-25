use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    'outer: for input in buf.lines().take_while(|&input| input != ".") {
        let mut open_close = Vec::new();

        for input_char in input.chars() {
            match input_char {
                '(' | '[' => open_close.push(input_char),
                ')' | ']' => {
                    if let Some(ch) = open_close.pop() {
                        if (input_char == ')' && ch != '(') || (input_char == ']' && ch != '[') {
                            writeln!(output, "no").unwrap();
                            continue 'outer;
                        }
                    } else {
                        writeln!(output, "no").unwrap();
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        writeln!(
            output,
            "{}",
            if open_close.is_empty() { "yes" } else { "no" }
        )
        .unwrap();
    }

    print!("{output}")
}
