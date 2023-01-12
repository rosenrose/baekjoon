use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let a: HashMap<_, _> = (0..n).map(|i| (input(), i)).collect();
    let mut index_arr = (0..n).map(|_| a.get(&input()).unwrap());

    let mut lis = vec![index_arr.next().unwrap()];

    for index in index_arr {
        if index > *lis.last().unwrap() {
            lis.push(index);
            continue;
        }

        let pos = lis.binary_search(&index).unwrap_or_else(|i| i);
        lis[pos] = index;
    }

    println!("{}", lis.len());
}
