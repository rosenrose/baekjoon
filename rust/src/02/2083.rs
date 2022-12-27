use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    while let Some(name) = input.next() {
        if name == "#" {
            return;
        }

        let (age, weight) = (
            parse_int(input.next().unwrap()),
            parse_int(input.next().unwrap()),
        );

        let class = if age > 17 || weight >= 80 {
            "Senior"
        } else {
            "Junior"
        };

        println!("{name} {class}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
