enum Tag {
    Open,
    Close,
    SelfClose,
}

use std::io;
use Tag::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        println!(
            "{}",
            if is_valid_xml(input) {
                "valid"
            } else {
                "invalid"
            }
        );
    }
}

fn is_valid_xml(input: &str) -> bool {
    let mut cursor = 0;
    let mut stack = Vec::new();

    for (tag_start, _) in input.match_indices('<') {
        let plain_text = &input[cursor..tag_start];

        if !is_valid_text(plain_text) {
            return false;
        }

        let Some(mut tag_end) = input[tag_start..].find('>') else {
            return false;
        };
        tag_end += tag_start;

        let tag = &input[tag_start + 1..tag_end];
        let tag_kind = if tag.starts_with('/') {
            Close
        } else if tag.ends_with('/') {
            SelfClose
        } else {
            Open
        };

        let tag_name = match tag_kind {
            Open => tag,
            Close => tag.trim_start_matches('/'),
            SelfClose => tag.trim_end_matches('/'),
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

        cursor = tag_end + 1;
    }

    stack.is_empty() && is_valid_text(&input[cursor..])
}

fn is_valid_text(input: &str) -> bool {
    if input.is_empty() {
        return true;
    }
    if input.contains(['<', '>']) {
        return false;
    }

    let mut cursor = 0;

    for (amp, _) in input.match_indices('&') {
        let plain_text = &input[cursor..amp];

        if !is_valid_text(plain_text) {
            return false;
        }

        let Some(mut semicolon) = input[amp..].find(';') else {
            return false;
        };
        semicolon += amp;

        let token = &input[amp + 1..semicolon];

        if matches!(token, "lt" | "gt" | "amp") {
            cursor = semicolon + 1;
            continue;
        }
        if !token.starts_with('x') {
            return false;
        }

        let hex = &token[1..];

        if !(hex.len() > 0 && (hex.len() & 1 == 0) && hex.chars().all(|ch| ch.is_ascii_hexdigit()))
        {
            return false;
        }

        cursor = semicolon + 1;
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
