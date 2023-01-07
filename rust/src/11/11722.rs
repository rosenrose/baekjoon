use std::cmp::Reverse;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut a = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());

    let mut lis = vec![a.next().unwrap()];

    for num in a {
        if num < *lis.last().unwrap() {
            lis.push(num);
            continue;
        }

        let pos = lis
            .binary_search_by_key(&Reverse(num), |&n| Reverse(n))
            .unwrap_or_else(|i| i);

        lis[pos] = num;
    }

    println!("{}", lis.len());
}
