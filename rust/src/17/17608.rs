use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut bars: Vec<_> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (mut max, mut count) = (bars.pop().unwrap(), 1);

    while let Some(height) = bars.pop() {
        if height > max {
            max = height;
            count += 1;
        }
    }

    println!("{count}");
}
