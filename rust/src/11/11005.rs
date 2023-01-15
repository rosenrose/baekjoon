use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (mut n, b) = (input.next().unwrap(), input.next().unwrap());
    let mut b_nums = Vec::new();

    while n != 0 {
        b_nums.push((n % b) as u8);
        n /= b;
    }

    b_nums.iter().rev().for_each(|&b_num| {
        if matches!(b_num, 0..=9) {
            print!("{b_num}")
        } else {
            print!("{}", ('A' as u8 + (b_num - 10)) as char)
        }
    });
}
