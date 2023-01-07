use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    'outer: for input in buf.lines().take_while(|&input| input != ".") {
        let mut open_close = Vec::new();

        for c in input.chars() {
            match c {
                '(' | '[' => open_close.push(c),
                ')' | ']' => match open_close.pop() {
                    Some(ch) => {
                        if (c == ')' && ch != '(') || (c == ']' && ch != '[') {
                            println!("no");
                            continue 'outer;
                        }
                    }
                    None => {
                        println!("no");
                        continue 'outer;
                    }
                },
                _ => (),
            };
        }

        println!("{}", if open_close.is_empty() { "yes" } else { "no" });
    }
}
