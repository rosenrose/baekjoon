use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut nums: Vec<_> = buf
        .lines()
        .skip(1)
        .flat_map(|input| {
            input.split(char::is_alphabetic).filter_map(|s| {
                (!s.is_empty()).then(|| {
                    let num = s.trim_start_matches('0');

                    if num.is_empty() {
                        "0"
                    } else {
                        num
                    }
                })
            })
        })
        .collect();

    nums.sort_by_key(|&n| (n.len(), n));

    for num in nums {
        println!("{num}");
    }
}
