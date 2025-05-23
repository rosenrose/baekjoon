use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut lis_temp = [0; MAX];
    lis_temp[0] = input.by_ref().skip(1).next().unwrap();
    let mut lis_temp_len = 1;

    for num in input {
        if num > lis_temp[lis_temp_len - 1] {
            lis_temp[lis_temp_len] = num;
            lis_temp_len += 1;
            continue;
        }

        let i = lis_temp[..lis_temp_len].partition_point(|&n| n < num);
        lis_temp[i] = num;
    }

    println!("{}", lis_temp_len);
}

// let n = input.next().unwrap() as usize;
// let mut nums = [0; MAX];

// for (i, num) in input.enumerate() {
//     nums[i] = num;
// }

// let mut lis_arr = [1; MAX];

// for start in 0..n - 1 {
//     for end in start + 1..n {
//         if nums[start] < nums[end] && lis_arr[end] < lis_arr[start] + 1 {
//             lis_arr[end] = lis_arr[start] + 1;
//         }
//     }
// }

// println!("{}", lis_arr[..n].iter().max().unwrap());
