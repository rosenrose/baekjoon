use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let a: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| input()).collect()).collect();
    input();

    let k = input();
    let b: Vec<Vec<_>> = (0..m).map(|_| (0..k).map(|_| input()).collect()).collect();

    for a_row in a {
        for i in 0..k as usize {
            let sum: i32 = a_row
                .iter()
                .enumerate()
                .map(|(j, a_num)| a_num * b[j][i])
                .sum();

            print!("{sum} ");
        }

        println!("");
    }
}
