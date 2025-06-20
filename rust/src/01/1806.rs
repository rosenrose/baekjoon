use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let (mut start, mut end) = (0, 0);
    let mut sum = nums[start];
    let mut min_len = usize::MAX;

    loop {
        if sum < m {
            end += 1;

            if end == n as usize {
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
