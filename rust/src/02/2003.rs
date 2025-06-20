use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let (mut start, mut end) = (0, 0);
    let mut sum = nums[start];
    let mut count = 0;

    loop {
        if sum == m {
            count += 1;
        }

        if sum < m {
            end += 1;

            if end == n {
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
