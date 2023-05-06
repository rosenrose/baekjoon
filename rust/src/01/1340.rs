fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.trim().split([' ', ',', ':']);
    let mut input = || input.next().unwrap();

    let (month, date, _, year) = (input(), parse_int(input()), input(), parse_int(input()));
    let month = match month {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => unreachable!(),
    };

    let (hour, minute) = (parse_int(input()), parse_int(input()));
    const DAY_MINUTES: i32 = 24 * 60;

    let total = if is_leap(year) { 366 } else { 365 } * DAY_MINUTES;
    let past: i32 = ((1..month).map(|m| get_days(m, year)).sum::<i32>() + date - 1) * DAY_MINUTES
        + hour * 60
        + minute;

    println!("{:.9}", (past as f64 / total as f64) * 100.0);
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

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
