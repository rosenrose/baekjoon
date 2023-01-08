use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut nums = [false; 2001];

    for num in input.skip(1) {
        nums[(num + 1000) as usize] = true;
    }

    for (num, _) in nums.iter().enumerate().filter(|(_, &b)| b) {
        print!("{} ", num as i32 - 1000);
    }
}
