use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let a: HashMap<_, _> = (0..n).map(|i| (input.next().unwrap(), i)).collect();
    let mut index_arr = (0..n).map(|_| a.get(&input.next().unwrap()).unwrap());

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
