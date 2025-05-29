use std::io;

const MAX: usize = 4;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let count = (1..=n).filter(is_hansu).count();

    println!("{count}");
}

fn is_hansu(num: &i32) -> bool {
    if *num < 100 {
        return true;
    }

    let mut digits = [0; MAX];
    let mut digits_len = 0;

    for digit in num.to_string().chars() {
        digits[digits_len] = digit as i8;
        digits_len += 1;
    }

    let diff = digits[0] - digits[1];

    (2..digits_len).all(|i| digits[i - 1] - digits[i] == diff)
}
