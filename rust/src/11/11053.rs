use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut lis_temp = vec![input.by_ref().skip(1).next().unwrap()];

    for num in input {
        if num > *lis_temp.last().unwrap() {
            lis_temp.push(num);
            continue;
        }

        let i = lis_temp.partition_point(|&n| n < num);
        lis_temp[i] = num;
    }

    println!("{}", lis_temp.len());
}

// let n = input.next().unwrap() as usize;
// let nums: Vec<_> = input.collect();
// let mut lis_arr = vec![1; n];

// for start in 0..n - 1 {
//     for end in start + 1..n {
//         if nums[start] < nums[end] && lis_arr[end] < lis_arr[start] + 1 {
//             lis_arr[end] = lis_arr[start] + 1;
//         }
//     }
// }

// println!("{}", lis_arr.iter().max().unwrap());
