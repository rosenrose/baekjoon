use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let mut ranks: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
        ranks.sort_unstable();

        let mut min_rank = i32::MAX;
        let mut count = 0;

        for (_, interview_rank) in ranks {
            if interview_rank < min_rank {
                count += 1;
            }

            min_rank = interview_rank.min(min_rank);
        }

        println!("{count}");
    }
}
