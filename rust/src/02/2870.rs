use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut nums: Vec<String> = buf
        .lines()
        .skip(1)
        .flat_map(|input| {
            input.split(char::is_alphabetic).filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                let mut s = s.to_string();

                while s.len() > 1 && s.chars().nth(0).unwrap() == '0' {
                    s.remove(0);
                }

                Some(s)
            })
        })
        .collect();

    nums.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for num in nums {
        println!("{num}");
    }
}
