use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (mut year, mut month) = (input.next().unwrap(), input.next().unwrap());
        month -= 1;

        if month <= 0 {
            month = 12;
            year -= 1;
        }

        let day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            #[rustfmt::skip]
            2 => if is_leap(year) { 29 } else { 28 },
            _ => 0,
        };

        writeln!(output, "{year} {month} {day}").unwrap();
    }

    print!("{output}");
}

fn is_leap(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}
