use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();

    match (
        nums[1] - nums[0],
        nums[2] - nums[1],
        nums[1] / nums[0],
        nums[2] / nums[1],
    ) {
        (d1, d2, ..) if d1 == d2 => println!("{}", nums.last().unwrap() + d1),
        (.., r1, r2) if r1 == r2 => println!("{}", nums.last().unwrap() * r1),
        _ => (),
    }
}
