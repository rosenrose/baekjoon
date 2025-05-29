use std::fmt::Write;
use std::io;

const MAKER_MAX: usize = 1000;
const CHECKER_MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut s = [0; MAKER_MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        s[i] = num;
    }

    let mut money = [0; MAKER_MAX + CHECKER_MAX];

    for i in 0..n {
        let mut given = s[i];

        for j in 0..n + m {
            let deploy = input.next().unwrap();

            money[j] += deploy;
            given -= deploy;
        }

        money[i] += given;
    }

    for &m in &money[..n + m] {
        write!(output, "{m} ").unwrap();
    }

    print!("{output}");
}
