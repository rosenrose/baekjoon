use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let (bookings, clean_time) = (parse_int(input()), parse_int(input()));
        let mut booking_times: Vec<_> = (0..bookings)
            .flat_map(|_| {
                let (_, enter_time, leave_time) = (
                    input(),
                    parse_time(input(), input()),
                    parse_time(input(), input()),
                );

                [(enter_time, true), (leave_time + clean_time, false)]
            })
            .collect();
        booking_times.sort_unstable();
        // println!("{booking_times:?}");
        let (mut count, mut max_count) = (0, 1);

        for (_, is_enter) in booking_times {
            if is_enter {
                count += 1;
            } else {
                count -= 1;
            }

            max_count = count.max(max_count);
        }

        writeln!(output, "{max_count}").unwrap();
    }

    print!("{output}");
}

fn parse_time(date: &str, time: &str) -> i32 {
    let mut it = date.split('-').chain(time.split(':')).map(parse_int);
    let mut token = || it.next().unwrap();
    let (year, month, date, hour, minlute) = (token(), token(), token(), token(), token());

    let days = (2013..year)
        .map(|y| if is_leap(y) { 366 } else { 365 })
        .sum::<i32>()
        + (1..month).map(|m| get_days(m, year)).sum::<i32>()
        + (date - 1);

    days * (24 * 60) + hour * 60 + minlute
}

fn get_days(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        #[rustfmt::skip]
        2 => if is_leap(year) { 29 } else { 28 },
        _ => unreachable!(),
    }
}

fn is_leap(year: i32) -> bool {
    year % 4 == 0
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
