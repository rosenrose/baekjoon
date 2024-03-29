use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    const YES: &str = "Yes";
    const NO: &str = "No";

    for (x, y) in (0..input()).map(|_| (input(), input())) {
        let is_time = matches!((x, y), (0..=23, 0..=59));
        let is_date = matches!(x, 1..=12)
            && (1..=(match x {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => 29,
                _ => unreachable!(),
            }))
                .contains(&y);

        writeln!(
            output,
            "{} {}",
            if is_time { YES } else { NO },
            if is_date { YES } else { NO }
        )
        .unwrap();
    }

    print!("{output}");
}
