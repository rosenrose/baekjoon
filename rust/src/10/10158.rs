use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (w, h, p, q, t) = (input(), input(), input(), input(), input());
    let (mut ant_x, mut ant_y) = (p, q);

    let mut time = t;
    let horizontal_dist = time.min(w - ant_x);

    ant_x += horizontal_dist;
    time -= horizontal_dist;
    time %= w * 2;

    ant_x = if time < w { ant_x - time } else { time % w };

    time = t;
    let vertical_dist = time.min(h - ant_y);

    ant_y += vertical_dist;
    time -= vertical_dist;
    time %= h * 2;

    ant_y = if time < h { ant_y - time } else { time % h };

    println!("{ant_x} {ant_y}");
}
