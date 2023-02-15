enum Tag {
    Open,
    Close,
    SelfClose,
}

use std::io;
use Tag::{Close, Open, SelfClose};

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

    while let Some(mut tag_start) = input[cursor..].find('<') {
        tag_start += cursor;
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
    if !input.contains("&") {
        return input.chars().all(|ch| matches!(ch as u8, 32..=127));
    }

    let mut cursor = 0;

    while let Some(mut amp_start) = input[cursor..].find('&') {
        amp_start += cursor;
        let plain_text = &input[cursor..amp_start];

        if !is_valid_text(plain_text) {
            return false;
        }

        let Some(mut semicolon) = input[amp_start..].find(';') else {
          return false;
      };
        semicolon += amp_start;

        let token = &input[amp_start + 1..semicolon];

        if token == "lt" || token == "gt" || token == "amp" {
            cursor = semicolon + 1;
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

        cursor = semicolon + 1;
    }

    is_valid_text(&input[cursor..])
}

fn is_valid_tag_name(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    input.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='z'))
}
