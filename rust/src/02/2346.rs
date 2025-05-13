use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut ballons: VecDeque<_> = input.enumerate().skip(1).collect();

    while let Some((idx, num)) = ballons.pop_front() {
        print!("{idx} ");

        if ballons.is_empty() {
            break;
        }

        if num > 0 {
            ballons.rotate_left((num as usize - 1) % ballons.len());
        } else {
            ballons.rotate_right(num.abs() as usize % ballons.len());
        }
    }
}
