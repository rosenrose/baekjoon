fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const M: i64 = 20_000_303;
    let mut pow_rem = 1;

    let remainder = buf.trim().as_bytes().iter().rev().fold(0, |acc, ch| {
        let num = (ch - b'0') as i64;
        let rem = (num * pow_rem) % M;
        pow_rem = (pow_rem * 10) % M;

        (acc + rem) % M
    });

    println!("{remainder}");
}
