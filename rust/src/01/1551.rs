use std::io::{stdin, Read};
use std::string::ToString;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    input.next();

    let k = parse_int(input.next().unwrap());
    let arr = input.next().unwrap();

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

    println!("{}", vec_join(&new_arr, ","))
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn vec_join<T>(vec: &Vec<T>, seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}
