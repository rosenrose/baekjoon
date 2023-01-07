use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let first_prize_rank = [(500, 1), (300, 3), (200, 6), (50, 10), (30, 15), (10, 21)];
    let second_prize_rank = [(512, 1), (256, 3), (128, 7), (64, 15), (32, 31)];

    for _ in 0..input.next().unwrap() {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());
        let mut total_prize = 0;

        for (prize, rank) in first_prize_rank {
            if a > 0 && a <= rank {
                total_prize += prize;
                break;
            }
        }

        for (prize, rank) in second_prize_rank {
            if b > 0 && b <= rank {
                total_prize += prize;
                break;
            }
        }

        writeln!(output, "{}", total_prize * 10000).unwrap();
    }

    print!("{output}");
}
