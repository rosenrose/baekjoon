use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input() as usize);
    let s: Vec<_> = (0..n).map(|_| input()).collect();
    let mut money = vec![0; n + m];

    for i in 0..n {
        let mut given = s[i];

        for j in 0..n + m {
            let deploy = input();

            money[j] += deploy;
            given -= deploy;
        }

        money[i] += given;
    }

    for m in money {
        write!(output, "{m} ").unwrap();
    }

    print!("{output}");
}
