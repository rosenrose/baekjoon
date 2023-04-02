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

    nums.sort_by(|a, b| (a.len(), a).cmp(&(b.len(), b)));

    for num in nums {
        println!("{num}");
    }
}
