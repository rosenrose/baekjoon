use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, m) = (input.next(), input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let (mut start, mut end) = (0, 0);
    let mut sum = nums[start];
    let mut count = 0;

    loop {
        if sum == m {
            count += 1;
        }

        if sum < m {
            end += 1;

            if end == nums.len() {
                break;
            }

            sum += nums[end];
        } else {
            sum -= nums[start];
            start += 1;
        }
    }

    println!("{count}");
}
