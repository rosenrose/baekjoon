use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, m) = (input.next(), input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let (mut i, mut j) = (0, 0);
    let mut sum = nums[i];
    let mut count = 0;

    loop {
        if sum == m {
            count += 1;
        }

        if sum < m {
            j += 1;

            if j == nums.len() {
                break;
            }

            sum += nums[j];
        } else {
            sum -= nums[i];
            i += 1;
        }
    }

    println!("{count}");
}
