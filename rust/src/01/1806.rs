use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, m] = [(); 2].map(|_| input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let (mut start, mut end) = (0, 0);
    let mut sum = nums[start];
    let mut min_len = usize::MAX;

    loop {
        if sum < m {
            end += 1;

            if end == nums.len() {
                break;
            }

            sum += nums[end];
        } else {
            min_len = min_len.min(end - start + 1);
            sum -= nums[start];
            start += 1;
        }
    }

    println!("{}", if min_len == usize::MAX { 0 } else { min_len });
}
