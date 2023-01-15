use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (h, _, n) in (0..input()).map(|_| (input(), input(), input())) {
        let floor = if n % h == 0 { h } else { n % h };
        let num_from_elevator = (n as f64 / h as f64).ceil() as i32;

        println!("{floor}{num_from_elevator:02}");
    }
}
