use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    for _ in 0..parse_int(input.next().unwrap()) {
        let n = parse_int(input.next().unwrap());
        let mut credits = 0;

        let sum: f64 = (0..n)
            .map(|_| {
                let (c, g) = (
                    parse_int(input.next().unwrap()),
                    input.next().unwrap().parse::<f64>().unwrap(),
                );

                credits += c;
                c as f64 * g
            })
            .sum();

        let gpa = sum / credits as f64;

        println!("{credits} {gpa:.1}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
