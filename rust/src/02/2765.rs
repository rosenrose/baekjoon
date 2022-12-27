use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());
    let mut input = || input.next().unwrap();

    const PI: f64 = 3.1415927;

    (1..)
        .map(|i| (i, (input(), input(), input())))
        .take_while(|&(_, (_, r, _))| r != 0.0)
        .for_each(|(i, (diameter, rotate, seconds))| {
            let distance = diameter * PI * rotate / (5280.0 * 12.0);
            let mph = distance * 3600.0 / seconds;

            let distance = (distance * 100.0).round() / 100.0;
            let mph = (mph * 100.0).round() / 100.0;

            println!("Trip #{i}: {distance:.2} {mph:.2}");
        });
}
