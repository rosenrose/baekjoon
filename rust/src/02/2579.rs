use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let scores: Vec<_> = input.skip(1).collect();

    match scores.len() {
        1 => println!("{}", scores[0]),
        2 => println!("{}", scores[0] + scores[1]),
        _ => {
            let mut max_scores = vec![
                scores[0],
                scores[0] + scores[1],
                scores[0].max(scores[1]) + scores[2],
            ];

            for i in 3..scores.len() {
                max_scores
                    .push(max_scores[i - 2].max(max_scores[i - 3] + scores[i - 1]) + scores[i]);
            }

            println!("{}", max_scores.last().unwrap());
        }
    }
}
