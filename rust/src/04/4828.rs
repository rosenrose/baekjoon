enum Tag {
    Open,
    Close,
    SelfClose,
}

use std::io;
use Tag::{Close, Open, SelfClose};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let tag_regex = Regex::new(r"<[^>]+>", false);
    let amp_regex = Regex::new(r"&(lt|gt|amp|x[0-9a-zA-Z]+);", false);

    for input in buf.lines() {
        println!(
            "{}",
            if is_valid_xml(input, &tag_regex, &amp_regex) {
                "valid"
            } else {
                "invalid"
            }
        );
    }
}

fn is_valid_xml(input: &str, tag_regex: &Regex, amp_regex: &Regex) -> bool {
    let mut cursor = 0;
    let mut stack = Vec::new();

    while let Some((mut start, mut end)) = tag_regex.find(&input[cursor..]) {
        start += cursor;
        end += cursor;

        let plain_text = &input[cursor..start];
        // println!("p: {plain_text}");
        if !is_valid_text(plain_text, amp_regex) {
            return false;
        }

        let tag = &input[start + 1..end - 1];
        // println!("t: {tag}");
        let tag_kind = if tag.starts_with('/') {
            Close
        } else if tag.ends_with('/') {
            SelfClose
        } else {
            Open
        };

        let tag_name = match tag_kind {
            Open => tag,
            Close => tag.strip_prefix('/').unwrap(),
            SelfClose => tag.strip_suffix('/').unwrap(),
        };

        if !is_valid_tag_name(tag_name) {
            return false;
        }

        match tag_kind {
            Open => stack.push(tag_name),
            Close => {
                let Some(opening_tag_name) = stack.pop() else {
                  return false;
              };

                if opening_tag_name != tag_name {
                    return false;
                }
            }
            SelfClose => (),
        }

        cursor = end;
    }

    stack.is_empty() && is_valid_text(&input[cursor..], amp_regex)
}

fn is_valid_text(input: &str, amp_regex: &Regex) -> bool {
    if input.is_empty() {
        return true;
    }
    if input.contains(['<', '>']) {
        return false;
    }

    let mut cursor = 0;

    while let Some((mut start, mut end)) = amp_regex.find(&input[cursor..]) {
        start += cursor;
        end += cursor;

        let plain_text = &input[cursor..start];

        if !is_valid_text(plain_text, amp_regex) {
            return false;
        }

        let token = &input[start + 1..end - 1];

        if matches!(token, "lt" | "gt" | "amp") {
            cursor = end;
            continue;
        }
        if !token.starts_with('x') {
            return false;
        }

        let hex = &token[1..];

        if !(hex.len() > 0
            && hex.len() % 2 == 0
            && hex
                .chars()
                .all(|ch| matches!(ch, '0'..='9' | 'a'..='f' | 'A'..='F')))
        {
            return false;
        }

        cursor = end;
    }

    if input[cursor..].contains('&') {
        return false;
    }

    input[cursor..]
        .chars()
        .all(|ch| matches!(ch as u8, 32..=127))
}

fn is_valid_tag_name(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    input.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='z'))
}
