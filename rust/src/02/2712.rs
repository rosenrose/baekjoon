fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let mut tokens = buf.split_whitespace();

        let value: f64 = tokens.next().unwrap().parse().unwrap();
        let unit = tokens.next().unwrap();

        let (converted, unit) = match unit {
            "kg" => (value * 2.2046, "lb"),
            "lb" => (value * 0.4536, "kg"),
            "l" => (value * 0.2642, "g"),
            "g" => (value * 3.7854, "l"),
            _ => (0.0, ""),
        };
        let converted = (converted * 10000.0).round() / 10000.0;

        println!("{converted:.4} {unit}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
