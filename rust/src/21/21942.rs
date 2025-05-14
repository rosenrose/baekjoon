use std::collections::HashMap;
use std::fmt::Write;
use std::io;

struct RentInfo<'a> {
    gears: Vec<(&'a str, i64)>,
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

    let mut rent_infos = HashMap::with_capacity(n as usize >> 1);

    for [date, time, gear, name] in (0..n).map(|_| [(); 4].map(|_| input())) {
        let info = rent_infos.entry(name).or_insert(RentInfo {
            gears: Vec::new(),
            fee: 0,
        });

        let time = parse_time(date, time);
        let Some((_, start)) = info.gears.iter().find(|(g, _)| g == &gear) else {
            info.gears.push((gear, time));
            continue;
        };

        let elapsed_minutes = time - start;

        if elapsed_minutes > max_minutes {
            info.fee += (elapsed_minutes - max_minutes) * penalty;
        }

        let idx = info.gears.iter().position(|(g, _)| g == &gear).unwrap();
        info.gears.remove(idx);
    }

    let mut payers: Vec<_> = rent_infos
        .iter()
        .filter_map(|(name, &RentInfo { fee, .. })| (fee > 0).then_some((name, fee)))
        .collect();

    if payers.len() == 0 {
        println!("-1");
        return;
    }

    payers.sort_unstable();

    for (name, fee) in payers {
        writeln!(output, "{name} {fee}").unwrap();
    }

    print!("{output}");
}

fn parse_time(date: &str, time: &str) -> i64 {
    let mut it = date
        .split('-')
        .skip(1)
        .chain(time.split(':'))
        .map(parse_int);
    let [month, date, hour, minute] = [(); 4].map(|_| it.next().unwrap());

    ((1..month).map(get_days).sum::<i64>() + date - 1) * 24 * 60 + hour * 60 + minute
}

fn get_days(month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => unreachable!(),
    }
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}
