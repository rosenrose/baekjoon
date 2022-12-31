use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();

    for i in 1..=parse_int(input.next().unwrap()) {
        let h = parse_int(input.next().unwrap());
        let h = input
            .next()
            .unwrap()
            .chars()
            .fold(h, |acc, c| if c == 'c' { acc + 1 } else { acc - 1 });

        println!("Data Set {i}:");
        println!("{h}\n");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
