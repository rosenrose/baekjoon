use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (mut y, mut m) = (0, 0);

    for time in input.skip(1) {
        y += ((time / 30) + 1) * 10;
        m += ((time / 60) + 1) * 15;
    }

    let (plan, cost) = match y.cmp(&m) {
        Ordering::Less => ("Y", y),
        Ordering::Equal => ("Y M", y),
        Ordering::Greater => ("M", m),
    };

    println!("{plan} {cost}");
}
