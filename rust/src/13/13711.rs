use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut a_index = [0; MAX];

    for i in 0..n {
        a_index[input()] = i;
    }

    let mut indices = (0..n).map(|_| a_index[input()]);
    let mut lis_temp = [0; MAX];
    lis_temp[0] = indices.next().unwrap();
    let mut lis_temp_len = 1;

    for index in indices {
        if index > lis_temp[lis_temp_len - 1] {
            lis_temp[lis_temp_len] = index;
            lis_temp_len += 1;
            continue;
        }

        let i = lis_temp[..lis_temp_len].partition_point(|&i| i < index);
        lis_temp[i] = index;
    }

    println!("{lis_temp_len}");
}
