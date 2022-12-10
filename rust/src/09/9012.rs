use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
