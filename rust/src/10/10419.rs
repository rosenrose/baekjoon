use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for d in input.skip(1) {
        let t: Vec<_> = (0..).take_while(|t| t * t + t <= d).collect();

        println!("{}", t.last().unwrap());
    }
}
