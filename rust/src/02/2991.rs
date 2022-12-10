use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let dog_atk_count = |time: i32, active: i32, rest: i32| {
        if time % (active + rest) < active {
            1
        } else {
            0
        }
    };

    if let [a, b, c, d, p, m, n] = input.collect::<Vec<_>>()[..] {
        for time in [p - 1, m - 1, n - 1] {
            println!("{}", dog_atk_count(time, a, b) + dog_atk_count(time, c, d));
        }
    }
}
