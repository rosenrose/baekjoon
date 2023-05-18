use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const R: i64 = 31;
    const M: i64 = 1_234_567_891;

    let input = buf.lines().next_back().unwrap();
    let hash = input.as_bytes().iter().enumerate().fold(0, |acc, (i, ch)| {
        let num = (ch - b'a' + 1) as i64;
        let mut rem = num % M;

        for _ in 0..i {
            rem = (rem * R) % M;
        }

        (acc + rem) % M
    });

    println!("{hash}");
}
