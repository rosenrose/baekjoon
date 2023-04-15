use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, k) = (input.next(), input.next().unwrap() as usize);
    let nums: Vec<_> = input.collect();
    let (mut sum, mut max_sum) = (0, 0);

    for (i, window) in nums.windows(k).enumerate() {
        if i == 0 {
            sum = window.iter().sum();
            max_sum = sum;
        } else {
            sum += window[k - 1];
            max_sum = sum.max(max_sum);
        }

        sum -= window[0];
    }

    println!("{max_sum}")
}
