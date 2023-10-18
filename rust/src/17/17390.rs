use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, q] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut nums: Vec<_> = input.by_ref().take(n).collect();
    nums.sort_unstable();

    for i in 1..n {
        nums[i] += nums[i - 1];
    }

    for [left, right] in (0..q).map(|_| [(); 2].map(|_| input.next().unwrap() as usize - 1)) {
        writeln!(
            output,
            "{}",
            nums[right] - if left == 0 { 0 } else { nums[left - 1] }
        )
        .unwrap();
    }

    print!("{output}");
}
