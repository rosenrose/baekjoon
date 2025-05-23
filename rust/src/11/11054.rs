use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let bitonic_len = (0..n)
        .map(|i| {
            let (mut lis_temp, mut lds_temp) = ([0; MAX], [0; MAX]);
            lis_temp[0] = nums[0];
            lds_temp[0] = nums[i];
            let (mut lis_temp_len, mut lds_temp_len) = (1, 1);

            for &num in &nums[..=i] {
                if num > lis_temp[lis_temp_len - 1] {
                    lis_temp[lis_temp_len] = num;
                    lis_temp_len += 1;
                    continue;
                }

                let lis_idx = lis_temp[..lis_temp_len].partition_point(|&n| n < num);
                lis_temp[lis_idx] = num;
            }

            for &num in &nums[i..n] {
                if num < lds_temp[lds_temp_len - 1] {
                    lds_temp[lds_temp_len] = num;
                    lds_temp_len += 1;
                    continue;
                }

                let lds_idx = lds_temp[..lds_temp_len].partition_point(|&n| n > num);
                lds_temp[lds_idx] = num;
            }

            lis_temp_len + lds_temp_len - 1
        })
        .max()
        .unwrap();

    println!("{bitonic_len}");
}
