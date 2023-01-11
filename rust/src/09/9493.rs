use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    while let (m @ 1.., a @ 1.., b @ 1..) = (input(), input(), input()) {
        let train_seconds = m as f64 * 3600.0 / a as f64;
        let plane_seconds = m as f64 * 3600.0 / b as f64;
        let diff = (train_seconds - plane_seconds).round() as i32;

        let s = diff % 60;
        let m = (diff / 60) % 60;
        let h = (diff / 60) / 60;

        println!("{h}:{m:02}:{s:02}");
    }
}
