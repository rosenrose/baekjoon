use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut sum = 0;

    buf.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, avg)| {
            let num = avg * (i as i32 + 1) - sum;
            print!("{num} ");

            sum += num;
        });
}
