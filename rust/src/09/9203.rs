use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let (bookings, clean_time) = (parse_int(input()), parse_int(input()));
        let mut booking_infos: Vec<_> = (0..bookings)
            .map(|_| {
                let (_, enter_time, leave_time) = (
                    input(),
                    parse_time(input(), input()),
                    parse_time(input(), input()),
                );

                ((enter_time, leave_time + clean_time), 1)
            })
            .collect();
        booking_infos.sort_unstable();
        // println!("{booking_infos:?}");
        let mut max_overlap = 1;

        for i in 0..booking_infos.len() {
            let ((_, cur_end), _) = booking_infos[i];

            for j in i + 1..booking_infos.len() {
                let ((next_start, _), _) = booking_infos[j];

                if next_start >= cur_end {
                    break;
                }

                booking_infos[j].1 += 1;
            }

            max_overlap = booking_infos[i].1.max(max_overlap);
        }

        writeln!(output, "{max_overlap}").unwrap();
    }

    print!("{output}");
}

fn parse_time(date: &str, time: &str) -> i32 {
    let mut tokens = date.split('-').chain(time.split(':')).map(parse_int);
    let mut token = || tokens.next().unwrap();
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
        _ => Default::default(),
    }
}

fn is_leap(year: i32) -> bool {
    year % 4 == 0
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
