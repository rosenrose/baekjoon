use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut nums: Vec<_> = buf
        .lines()
        .skip(1)
        .flat_map(|input| {
            input.split(char::is_alphabetic).filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                let mut s = s.to_owned();

                while s.len() > 1 && s.chars().nth(0) == Some('0') {
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
