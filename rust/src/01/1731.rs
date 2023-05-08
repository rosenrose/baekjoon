use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let (diff, ratio) = (nums[1] - nums[0], nums[1] / nums[0]);
    let last = nums.last().unwrap();

    if diff == nums[2] - nums[1] {
        println!("{}", last + diff);
        return;
    }

    println!("{}", last * ratio);
}
