use std::collections::HashMap;
use std::fmt::Write;
use std::io;

#[derive(Copy, Clone)]
struct Datetime {
    month: i8,
    date: i8,
    hour: i8,
    minute: i8,
}
struct RentInfo<'a> {
    gears: HashMap<&'a str, Datetime>,
    fee: i64,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, rent_term, penalty) = (parse_int(input()), input(), parse_int(input()));
    let rent_term: Vec<_> = rent_term.split(['/', ':']).map(parse_int).collect();
    let max_minutes = rent_term[0] * 24 * 60 + rent_term[1] * 60 + rent_term[2];

    let mut rent_infos = HashMap::new();

    for (date, time, gear, name) in (0..n).map(|_| (input(), input(), input(), input())) {
        let info = rent_infos.entry(name).or_insert(RentInfo {
            gears: HashMap::new(),
            fee: 0,
        });

        let dt = parse_datetime(date, time);
        let Some(&start) = info.gears.get(gear) else {
            info.gears.insert(gear, dt);
            continue;
        };

        let minutes = elapsed_minutes(start, dt);

        if minutes > max_minutes {
            info.fee += (minutes - max_minutes) * penalty;
        }

        info.gears.remove(gear);
    }

    let mut payers: Vec<_> = rent_infos
        .iter()
        .filter_map(|(name, &RentInfo { fee, .. })| (fee > 0).then_some((name, fee)))
        .collect();

    if payers.len() == 0 {
        println!("-1");
        return;
    }

    payers.sort_unstable_by_key(|(&name, _)| name);

    for (name, fee) in payers {
        writeln!(output, "{name} {fee}").unwrap();
    }

    print!("{output}");
}

fn parse_datetime(date: &str, time: &str) -> Datetime {
    let mut tokens = date
        .split('-')
        .skip(1)
        .chain(time.split(':'))
        .map(|s| parse_int(s) as i8);
    let mut token = || tokens.next().unwrap();

    Datetime {
        month: token(),
        date: token(),
        hour: token(),
        minute: token(),
    }
}

fn elapsed_minutes(start: Datetime, end: Datetime) -> i64 {
    let past_minutes = |dt: Datetime| {
        ((1..dt.month).map(get_days).sum::<i64>() + dt.date as i64 - 1) * 24 * 60
            + dt.hour as i64 * 60
            + dt.minute as i64
    };

    past_minutes(end) - past_minutes(start)
}

fn get_days(month: i8) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => Default::default(),
    }
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}
