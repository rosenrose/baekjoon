use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut a = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());

    let mut lis = vec![a.next().unwrap()];

    for num in a {
        if num > *lis.last().unwrap() {
            lis.push(num);
            continue;
        }

        let pos = lis.binary_search(&num).unwrap_or_else(|i| i);
        lis[pos] = num;
    }

    println!("{}", lis.len());
}
