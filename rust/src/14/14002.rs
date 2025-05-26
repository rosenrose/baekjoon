use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut lis_temp = [0; MAX];
    lis_temp[0] = input.by_ref().skip(1).next().unwrap();
    let mut lis_temp_len = 1;

    let mut num_indices = [(0, 0); MAX];
    num_indices[0] = (lis_temp[0], 0);
    let mut num_indices_len = 1;

    for num in input {
        if num > lis_temp[lis_temp_len - 1] {
            num_indices[num_indices_len] = (num, lis_temp_len);
            num_indices_len += 1;

            lis_temp[lis_temp_len] = num;
            lis_temp_len += 1;
            continue;
        }

        let i = lis_temp[..lis_temp_len].partition_point(|&n| n < num);
        lis_temp[i] = num;

        num_indices[num_indices_len] = (num, i);
        num_indices_len += 1;
    }

    let lis_len = lis_temp_len;
    let mut lis = [0; MAX];

    for &(num, index) in num_indices[..num_indices_len].iter().rev() {
        if index + 1 == lis_temp_len {
            lis[index] = num;
            lis_temp_len -= 1;
        }

        if lis_temp_len == 0 {
            break;
        }
    }

    println!("{lis_len}");

    for num in &lis[..lis_len] {
        print!("{num} ");
    }
}
