use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let first_prize_rank = [(500, 1), (300, 3), (200, 6), (50, 10), (30, 15), (10, 21)];
    let second_prize_rank = [(512, 1), (256, 3), (128, 7), (64, 15), (32, 31)];

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        let (prize1, _) = first_prize_rank
            .iter()
            .find(|&&(_, rank)| a > 0 && a <= rank)
            .unwrap_or(&(0, 0));
        let (prize2, _) = second_prize_rank
            .iter()
            .find(|&&(_, rank)| b > 0 && b <= rank)
            .unwrap_or(&(0, 0));

        writeln!(output, "{}", (prize1 + prize2) * 10000).unwrap();
    }

    print!("{output}");
}
