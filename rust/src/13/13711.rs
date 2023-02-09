use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input();
    let a: HashMap<_, _> = (0..n).map(|i| (input(), i)).collect();
    let mut index_arr = (0..n).map(|_| a[&input()]);

    let mut lis_temp = vec![index_arr.next().unwrap()];

    for index in index_arr {
        if index > *lis_temp.last().unwrap() {
            lis_temp.push(index);
            continue;
        }

        let pos = lis_temp.binary_search(&index).unwrap_or_else(|i| i);
        lis_temp[pos] = index;
    }

    println!("{}", lis_temp.len());
}
