use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    const PI: f64 = 3.1415927;

    buf.lines().enumerate().for_each(|(i, line)| {
        if let [diameter, rotate, seconds] = parse_float_vec(line)[..] {
            if rotate == 0.0 {
                return;
            }

            let distance = diameter * PI * rotate / (5280.0 * 12.0);
            let mph = distance * 3600.0 / seconds;

            let distance = (distance * 100.0).round() / 100.0;
            let mph = (mph * 100.0).round() / 100.0;

            println!("Trip #{}: {distance:.2} {mph:.2}", i + 1);
        }
    });
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
