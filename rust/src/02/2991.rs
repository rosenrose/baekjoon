use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (a, b, c, d) = (input(), input(), input(), input());
    let (p, m, n) = (input(), input(), input());

    let dog_atk_count =
        |time: i32, active: i32, rest: i32| u8::from(time % (active + rest) < active);

    for time in [p - 1, m - 1, n - 1] {
        println!("{}", dog_atk_count(time, a, b) + dog_atk_count(time, c, d));
    }
}
