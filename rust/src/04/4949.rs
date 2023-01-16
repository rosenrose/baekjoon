use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    'outer: for input in buf.lines().take_while(|&input| input != ".") {
        let mut open_close = Vec::new();

        for c in input.chars() {
            match c {
                '(' | '[' => open_close.push(c),
                ')' | ']' => match open_close.pop() {
                    Some(ch) => {
                        if (c == ')' && ch != '(') || (c == ']' && ch != '[') {
                            writeln!(output, "no").unwrap();
                            continue 'outer;
                        }
                    }
                    None => {
                        writeln!(output, "no").unwrap();
                        continue 'outer;
                    }
                },
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
