use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let min_time = (0..input()).fold(i32::MAX, |min, _| {
        let (a, b) = (input(), input());

        if a > b {
            return min;
        }

        let time = a.max(b);
        time.min(min)
    });

    println!("{}", if min_time == i32::MAX { -1 } else { min_time });
}
