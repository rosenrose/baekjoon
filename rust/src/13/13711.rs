use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut a_index = vec![0; n + 1];

    for i in 0..n {
        a_index[input()] = i;
    }

    let mut indices = (0..n).map(|_| a_index[input()]);
    let mut lis_temp = vec![indices.next().unwrap()];

    for index in indices {
        if index > *lis_temp.last().unwrap() {
            lis_temp.push(index);
            continue;
        }

        let i = lis_temp.partition_point(|&i| i < index);
        lis_temp[i] = index;
    }

    println!("{}", lis_temp.len());
}
