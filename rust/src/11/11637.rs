use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let n = input();
        let (mut total, mut max_vote) = (0, 0);

        let candidates: Vec<_> = (0..n)
            .map(|i| {
                let votes = input();
                total += votes;
                max_vote = votes.max(max_vote);

                votes
            })
            .collect();

        let winners: Vec<_> = candidates
            .iter()
            .enumerate()
            .filter_map(|(i, &vote)| (vote == max_vote).then_some(i + 1))
            .collect();

        if winners.len() > 1 {
            writeln!(output, "no winner").unwrap();
            continue;
        }

        writeln!(
            output,
            "{} winner {}",
            if max_vote > total / 2 {
                "majority"
            } else {
                "minority"
            },
            winners[0]
        )
        .unwrap();
    }

    print!("{output}");
}
