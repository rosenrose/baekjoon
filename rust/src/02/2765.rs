use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    const PI: f64 = 3.1415927;

    (1..)
        .map(|i| (i, (input(), input(), input())))
        .take_while(|&(_, (_, r, _))| r != 0.0)
        .for_each(|(i, (diameter, rotate, seconds))| {
            let distance = diameter * PI * rotate / (5280.0 * 12.0);
            let mph = distance * 3600.0 / seconds;

            println!("Trip #{i}: {distance:.2} {mph:.2}");
        });
}
