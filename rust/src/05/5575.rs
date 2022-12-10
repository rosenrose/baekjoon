use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..3 {
        if let [on_hour, on_minute, on_second, off_hour, off_minute, off_second] =
            (0..6).map(|_| input.next().unwrap()).collect::<Vec<_>>()[..]
        {
            let mut hours = off_hour - on_hour;
            let mut minutes = off_minute - on_minute;
            let mut seconds = off_second - on_second;

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
}
