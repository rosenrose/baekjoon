use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const ILLEGAL: &str = "illegal";

    let is_closing = |tag: &str| tag.starts_with('/');
    let is_self_closing = |tag: &str| tag.ends_with('/');

    'outer: for input in buf.lines().take_while(|&input| input != "#") {
        let mut stack = Vec::<&str>::new();

        for tag in input
            .split_inclusive(['<', '>'])
            .filter_map(|s| s.strip_suffix('>'))
        {
            // println!("{tag}");
            if is_self_closing(tag) {
                continue;
            }

            if is_closing(tag) && tag.contains('=') {
                println!("{ILLEGAL}");
                continue 'outer;
            }

            if is_closing(tag) {
                let Some(opening) = stack.pop() else {
                    println!("{ILLEGAL}");
                    continue 'outer;
                };

                let opening = if let Some((name, _attr)) = opening.split_once(' ') {
                    name
                } else {
                    opening
                };
                let closing = tag.strip_prefix('/').unwrap();

                if opening != closing {
                    println!("{ILLEGAL}");
                    continue 'outer;
                }
            } else {
                stack.push(tag);
            }
        }

        println!("{}", if stack.is_empty() { "legal" } else { ILLEGAL });
    }
}
