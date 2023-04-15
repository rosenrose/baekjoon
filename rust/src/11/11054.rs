use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let bitonic_len = (0..nums.len())
        .map(|i| {
            let mut lis_temp = vec![nums[0]];
            let mut lds_temp = vec![nums[i]];

            for &num in &nums[..=i] {
                if num > *lis_temp.last().unwrap() {
                    lis_temp.push(num);
                    continue;
                }

                let lis_idx = lis_temp.partition_point(|&n| n < num);
                lis_temp[lis_idx] = num;
            }

            for &num in &nums[i..] {
                if num < *lds_temp.last().unwrap() {
                    lds_temp.push(num);
                    continue;
                }

                let lds_idx = lds_temp.partition_point(|&n| n > num);
                lds_temp[lds_idx] = num;
            }

            lis_temp.len() + lds_temp.len() - 1
        })
        .max()
        .unwrap();

    println!("{bitonic_len}");
}
