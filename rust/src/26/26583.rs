use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let nums: Vec<i32> = input.split(' ').flat_map(str::parse).collect();
        let scaled = nums.iter().enumerate().map(|(i, num)| {
            let left = if i == 0 { 1 } else { nums[i - 1] };
            let right = nums.get(i + 1).unwrap_or(&1);

            num * left * right
        });

        for num in scaled {
            print!("{num} ");
        }
        println!("");
    }
}
