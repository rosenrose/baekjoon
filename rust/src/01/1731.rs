use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let (diff, ratio) = (nums[1] - nums[0], nums[1] / nums[0]);
    let last = nums[n - 1];

    if diff == nums[2] - nums[1] {
        println!("{}", last + diff);
        return;
    }

    println!("{}", last * ratio);
}
