use std::io;

const MAX: usize = 1_000_000;

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

    println!("{lis_temp_len}");
}
