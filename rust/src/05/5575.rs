use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..3 {
        let (on_hour, on_minute, on_second, off_hour, off_minute, off_second) =
            (input(), input(), input(), input(), input(), input());

        let (mut hours, mut minutes, mut seconds) = (
            off_hour - on_hour,
            off_minute - on_minute,
            off_second - on_second,
        );

        if seconds < 0 {
            seconds += 60;
            minutes -= 1;
        }

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        println!("{hours} {minutes} {seconds}");
    }
}
