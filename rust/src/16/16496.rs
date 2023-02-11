use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut nums: Vec<_> = input.skip(1).collect();

    nums.sort_by(|&a, &b| {
        if a.len() == b.len() {
            b.cmp(&a)
        } else {
            [b, a].concat().cmp(&[a, b].concat())
        }
    });

    if nums[0] == "0" {
        println!("0");
    } else {
        println!("{}", nums.join(""));
    }
}
