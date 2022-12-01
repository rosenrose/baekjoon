fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const M: i64 = 20000303;
    let mut exp_remainder = 1 % M;

    let remainder = buf.trim().chars().rev().fold(0, |acc, c| {
        let num = c.to_digit(10).unwrap() as i64;
        let rem = ((num % M) * (exp_remainder % M)) % M;

        exp_remainder = ((exp_remainder % M) * (10 % M)) % M;

        (acc % M + rem % M) % M
    });

    println!("{remainder}");
}
