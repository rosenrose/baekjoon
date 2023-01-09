fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const M: i64 = 20000303;
    let mut exp_rem = 1;

    let remainder = buf.trim().chars().rev().fold(0, |acc, c| {
        let num = c as i64 - '0' as i64;

        let rem = (num * exp_rem) % M;
        exp_rem = (exp_rem * 10) % M;

        (acc + rem) % M
    });

    println!("{remainder}");
}
