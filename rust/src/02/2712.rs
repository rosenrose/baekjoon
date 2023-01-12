use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (value, unit) in (0..n).map(|_| (input().parse::<f64>().unwrap(), input())) {
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
