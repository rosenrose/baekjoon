use std::io;

const MAX: usize = 20000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    nums[..n].sort();

    if n % 2 == 1 {
        println!("{}", nums[n / 2]);
        return;
    }

    let (a, b) = (nums[n / 2 - 1], nums[n / 2]);
    let diff_a = nums[..n].iter().map(|num| num.abs_diff(a)).sum::<u32>();
    let diff_b = nums[..n].iter().map(|num| num.abs_diff(b)).sum::<u32>();

    println!("{}", if diff_a <= diff_b { a } else { b });
}
