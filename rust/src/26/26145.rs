use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input() as usize);
    let s: Vec<_> = (0..n).map(|_| input()).collect();

    let money = (0..n).fold(vec![0; n + m], |mut acc, i| {
        let given = (0..n + m).fold(s[i], |given_acc, j| {
            let deploy = input();
            acc[j] += deploy;

            given_acc - deploy
        });

        acc[i] += given;
        acc
    });

    for m in money {
        write!(output, "{m} ").unwrap();
    }

    print!("{output}");
}
