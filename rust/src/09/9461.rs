use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for n in input.skip(1) {
        if n <= 3 {
            println!("1");
            continue;
        }
        if n == 4 {
            println!("2");
            continue;
        }

        let (mut a, mut b, mut c, mut d, mut e) = (1_i64, 1, 1, 2, 2);

        for _ in 5..n {
            (a, b, c, d, e) = (b, c, d, e, a + e);
        }

        println!("{e}");
    }
}
