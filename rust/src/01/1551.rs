use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (_, k) = (input(), parse_int(input()));
    let arr = input();

    if k == 0 {
        println!("{arr}");
        return;
    }

    let arr: Vec<_> = arr.split(',').map(parse_int).collect();
    let mut new_arr: Vec<_> = (1..arr.len()).map(|i| arr[i] - arr[i - 1]).collect();

    for _ in 0..k - 1 {
        new_arr = (1..new_arr.len())
            .map(|i| new_arr[i] - new_arr[i - 1])
            .collect();
    }

    println!("{}", format!("{new_arr:?}").replace(['[', ']', ' '], ""));
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
