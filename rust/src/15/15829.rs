use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    const R: i64 = 31;
    const M: i64 = 1234567891;

    let hash = buf
        .lines()
        .next_back()
        .unwrap()
        .char_indices()
        .fold(0, |acc, (i, ch)| {
            let num = (ch as u8 - 'a' as u8 + 1) as i64;
            let mut rem = num % M;

            for _ in 0..i {
                rem = ((rem % M) * (R % M)) % M;
            }

            (acc % M + rem % M) % M
        });

    println!("{hash}");
}
