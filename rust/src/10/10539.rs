use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut sum = 0;

    buf.split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<i32>)
        .enumerate()
        .for_each(|(i, avg)| {
            let num = avg * (i as i32 + 1) - sum;
            print!("{num} ");

            sum += num;
        });
}
