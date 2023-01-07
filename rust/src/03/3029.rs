use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let parse_time = |s: &str| -> Vec<i32> { s.split(':').map(|s| s.parse().unwrap()).collect() };

    let current = parse_time(input.next().unwrap());
    let mut end = parse_time(input.next().unwrap());

    if end <= current {
        end[0] += 24;
    }

    let delta: Vec<_> = end.iter().zip(current).map(|(e, c)| e - c).collect();

    if let [mut left_hour, mut left_minute, mut left_second] = delta[..] {
        if left_second < 0 {
            left_second += 60;
            left_minute -= 1;
        }

        if left_minute < 0 {
            left_minute += 60;
            left_hour -= 1;
        }

        println!("{left_hour:02}:{left_minute:02}:{left_second:02}");
    }
}
