use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, q) = input().split_once(' ').unwrap();
    let mut logs = vec![Vec::new(); 7];

    for _ in 0..parse_int(n) {
        let (time, level) = input().split_once('#').unwrap();
        logs[parse_int(level) as usize].push(parse_time(time));
    }

    for _ in 0..parse_int(q) {
        let mut tokens = input().split('#');
        let (start, end, level) = (
            parse_time(tokens.next().unwrap()),
            parse_time(tokens.next().unwrap()),
            parse_int(tokens.next().unwrap()) as usize,
        );

        let count: usize = (level..=6)
            .map(|lv| {
                let start_idx = logs[lv].partition_point(|&log| log < start);
                let end_idx = logs[lv].partition_point(|&log| log <= end);

                end_idx - start_idx
            })
            .sum();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn parse_time(input: &str) -> i32 {
    let mut tokens = input.split(['-', ' ', ':']).map(parse_int);
    let mut token = || tokens.next().unwrap();
    let (year, month, date, hour, minlute, second) =
        (token(), token(), token(), token(), token(), token());

    let days = (2000..year)
        .map(|y| if is_leap(y) { 366 } else { 365 })
        .sum::<i32>()
        + (1..month).map(|m| get_days(m, year)).sum::<i32>()
        + (date - 1);

    days * (24 * 60 * 60) + hour * 60 * 60 + minlute * 60 + second
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

fn get_days(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        #[rustfmt::skip]
        2 => if is_leap(year) { 29 } else { 28 },
        _ => Default::default(),
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
