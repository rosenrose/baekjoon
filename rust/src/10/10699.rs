use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    const DAY: u64 = 60 * 60 * 24;
    const KST: u64 = 60 * 60 * 9;

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + KST;

    let mut year = 1970;
    let mut days = duration / DAY;

    loop {
        let day_count = if is_leap(year) { 366 } else { 365 };

        if days < day_count {
            break;
        }

        days -= day_count;
        year += 1;
    }

    let mut month = 0;

    for m in 1..=12 {
        let day_count = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            #[rustfmt::skip]
            2 => if is_leap(year) { 29 } else { 28 },
            _ => 0,
        };

        if days < day_count {
            month = m;
            break;
        }

        days -= day_count;
    }

    let day = 1 + days;

    println!("{year}-{month:02}-{day:02}");
}

fn is_leap(year: u64) -> bool {
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
