use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (mut hour, mut minute, mut second) = (input(), input(), input());
    let mut time = input();

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
