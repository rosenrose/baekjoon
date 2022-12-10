use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    if let [mut hour, mut minute, mut second, mut time] = input.collect::<Vec<_>>()[..] {
        second += time % 60;
        minute += second / 60;
        second %= 60;

        time -= time % 60;

        minute += (time % 3600) / 60;
        hour += minute / 60;
        minute %= 60;

        time -= time % 3600;

        hour += time / 3600;
        hour %= 24;

        println!("{hour} {minute} {second}");
    }
}
