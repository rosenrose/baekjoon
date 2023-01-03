use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let value: f64 = input.next().unwrap().parse().unwrap();
        let unit = input.next().unwrap();

        let (converted, unit) = match unit {
            "kg" => (value * 2.2046, "lb"),
            "lb" => (value * 0.4536, "kg"),
            "l" => (value * 0.2642, "g"),
            "g" => (value * 3.7854, "l"),
            _ => (0.0, ""),
        };

        println!("{converted:.4} {unit}");
    }
}
