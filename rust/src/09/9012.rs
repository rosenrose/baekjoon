use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    'outer: for input in buf.lines().skip(1) {
        let mut stack = Vec::new();

        for c in input.chars() {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if stack.pop().is_none() {
                        println!("NO");
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        println!("{}", if stack.is_empty() { "YES" } else { "NO" });
    }
}
