use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut line = String::new();
    const MAX_WIDTH: usize = 80;

    let flush_line = |line: &mut String| {
        println!("{}", line.trim_end());
        line.clear();
    };

    for word in buf.split_ascii_whitespace() {
        match word {
            "<br>" => flush_line(&mut line),
            "<hr>" => {
                if !line.is_empty() {
                    flush_line(&mut line);
                }
                println!("{}", "-".repeat(MAX_WIDTH));
            }
            _ => {
                if line.len() + word.len() > MAX_WIDTH {
                    flush_line(&mut line);
                }

                line.push_str(word);
                line.push(' ');
            }
        }
    }

    flush_line(&mut line);
}
