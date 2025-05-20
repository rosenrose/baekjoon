use std::io;

const MAX: usize = 300;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut scores = [0; MAX];

    for (i, num) in input.enumerate() {
        scores[i] = num;
    }

    match scores.len() {
        1 => println!("{}", scores[0]),
        2 => println!("{}", scores[0] + scores[1]),
        _ => {
            let mut max_scores = [0; MAX];
            max_scores[0] = scores[0];
            max_scores[1] = scores[0] + scores[1];
            max_scores[2] = scores[0].max(scores[1]) + scores[2];

            for i in 3..n {
                max_scores[i] =
                    max_scores[i - 2].max(max_scores[i - 3] + scores[i - 1]) + scores[i];
            }

            println!("{}", max_scores[n - 1]);
        }
    }
}
