use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const ILLEGAL: &str = "Illegal";

    'outer: for input in buf.lines().take_while(|&input| input != "#") {
        let mut parens = Vec::new();

        for ch in input.chars() {
            match ch {
                '(' | '[' | '{' => parens.push(ch),
                ')' | ']' | '}' => {
                    let Some(last) = parens.pop() else {
                        println!("{ILLEGAL}");
                        continue 'outer;
                    };

                    if !matches!((last, ch), ('(', ')') | ('[', ']') | ('{', '}')) {
                        println!("{ILLEGAL}");
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        println!("{}", if parens.is_empty() { "Legal" } else { ILLEGAL });
    }
}
