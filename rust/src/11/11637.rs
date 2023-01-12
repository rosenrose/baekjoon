use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let n = input();
        let (mut total, mut max) = (0, 0);

        let candidates: Vec<_> = (1..=n)
            .map(|i| {
                let votes = input();
                total += votes;
                max = votes.max(max);

                (i, votes)
            })
            .collect();
        let winners: Vec<_> = candidates.iter().filter(|&&(_, v)| v == max).collect();

        if winners.len() > 1 {
            writeln!(output, "no winner").unwrap();
            continue;
        }

        writeln!(
            output,
            "{} winner {}",
            if max > total / 2 {
                "majority"
            } else {
                "minority"
            },
            winners[0].0
        )
        .unwrap();
    }

    print!("{output}");
}
