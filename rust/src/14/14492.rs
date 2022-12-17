use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let a: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| input.next().unwrap() == "1").collect())
        .collect();
    let b: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| input.next().unwrap() == "1").collect())
        .collect();

    let count: i32 = a
        .iter()
        .flat_map(|a_row| {
            (0..n).map(|i| {
                if a_row
                    .iter()
                    .enumerate()
                    .map(|(j, a_num)| a_num & b[j][i])
                    .any(|and| and)
                {
                    1
                } else {
                    0
                }
            })
        })
        .sum();

    println!("{count}");
}
