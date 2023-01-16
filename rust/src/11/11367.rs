use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for (name, grade) in (0..parse_int(input())).map(|_| (input(), parse_int(input()))) {
        println!(
            "{name} {}",
            match grade {
                97..=100 => "A+",
                90..=96 => "A",
                87..=89 => "B+",
                80..=86 => "B",
                77..=79 => "C+",
                70..=76 => "C",
                67..=69 => "D+",
                60..=66 => "D",
                0..=59 => "F",
                _ => "",
            }
        );
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
