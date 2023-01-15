use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input();
    const MAX: i32 = 10000;

    let (x_min, x_max, y_min, y_max) =
        (0..n).fold((MAX, -MAX, MAX, -MAX), |(x_min, x_max, y_min, y_max), _| {
            let (x, y) = (input(), input());

            (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
        });

    println!("{}", x_min.abs_diff(x_max) * y_min.abs_diff(y_max));
}
