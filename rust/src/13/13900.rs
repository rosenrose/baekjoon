use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let mut sum_accum = vec![0; n];

    let nums: Vec<_> = input
        .enumerate()
        .map(|(i, num)| {
            sum_accum[i] = num + if i == 0 { 0 } else { sum_accum[i - 1] };
            num
        })
        .collect();
    // println!("{nums:?} {sum_accum:?}");
    let product_sum: i64 = (1..n).map(|i| nums[i] * sum_accum[i - 1]).sum();

    println!("{product_sum}");
}
