use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    for i in 1..=parse_int(input()) {
        let h = parse_int(input());
        let h = input()
            .chars()
            .fold(h, |acc, c| if c == 'c' { acc + 1 } else { acc - 1 });

        println!("Data Set {i}:");
        println!("{h}\n");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
