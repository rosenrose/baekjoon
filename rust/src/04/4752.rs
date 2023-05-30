use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().map(str::trim) {
        let (b, right) = input.split_once(' ').unwrap();
        let Some((e, text)) = right.split_once(' ') else {
            return;
        };

        let (begin, end) = (parse_int(b), parse_int(e));
        let selected = &text[begin..end];

        let outer_tags = get_tags(&text[..begin]);
        let inner_tags = get_tags(selected);
        // println!("{outer_tags:?} {inner_tags:?}");
        for out_tag in &outer_tags {
            print!("<{out_tag}>");
        }

        print!("{selected}");

        for in_tag in inner_tags.iter().rev() {
            print!("</{in_tag}>");
        }

        for out_tag in outer_tags.iter().rev() {
            let closing = format!("</{out_tag}>");

            if selected.contains(&closing) {
                continue;
            }

            print!("{closing}");
        }
        println!("");
    }
}

fn get_tags(input: &str) -> Vec<String> {
    let mut stack = Vec::new();
    let mut tag = String::new();
    let mut is_tag = false;
    let mut is_closing = false;

    for ch in input.chars() {
        if ch == '<' {
            is_tag = true;
            continue;
        }
        if !is_tag {
            continue;
        }

        match ch {
            '/' => is_closing = true,
            '>' => {
                if is_closing {
                    stack.pop();
                    is_closing = false;
                } else {
                    stack.push(tag.clone());
                }

                tag.clear();
                is_tag = false;
            }
            _ => {
                if !is_closing {
                    tag.push(ch);
                }
            }
        }
    }

    stack
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
