use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for n in input.take_while(|&n| n != 0) {
        let squares: i32 = (1..=n).map(|i| (n - i + 1).pow(2)).sum();

        println!("{squares}");
    }
}
