use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, q) = (input() as usize, input());
    let mut nums: Vec<_> = (0..n).map(|_| input()).collect();
    nums.sort_unstable();

    for i in 1..n {
        nums[i] += nums[i - 1];
    }

    for (left, right) in (0..q).map(|_| (input() as usize - 1, input() as usize - 1)) {
        writeln!(
            output,
            "{}",
            nums[right] - if left == 0 { 0 } else { nums[left - 1] }
        )
        .unwrap();
    }

    print!("{output}");
}
