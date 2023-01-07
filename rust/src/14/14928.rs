use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const M: i64 = 20000303;
    let mut exp_rem = 1 % M;

    let remainder = buf.trim().chars().rev().fold(0, |acc, c| {
        let num = c.to_digit(10).unwrap() as i64;
        let rem = ((num % M) * (exp_rem % M)) % M;

        exp_rem = ((exp_rem % M) * (10 % M)) % M;

        (acc % M + rem % M) % M
    });

    println!("{remainder}");
}
