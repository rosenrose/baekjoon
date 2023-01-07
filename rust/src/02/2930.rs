use std::io::{read_to_string, stdin};

fn main() {
    let buf = read_to_string(stdin()).unwrap();
    let mut input = buf.lines();
    input.next();

    let my_rps = input.next().unwrap();
    let friends: Vec<_> = input.skip(1).collect();

    let (score, max_score) = my_rps
        .char_indices()
        .map(|(i, my)| {
            let mut score = 0;
            let max = ['R', 'P', 'S']
                .iter()
                .map(|&best| {
                    let s: i32 = friends
                        .iter()
                        .map(|&friend| get_rps_score(best, friend.chars().nth(i).unwrap()))
                        .sum();

                    if best == my {
                        score = s;
                    }

                    s
                })
                .max()
                .unwrap();

            (score, max)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("{score}\n{max_score}");
}

fn get_rps_score(a: char, b: char) -> i32 {
    match (a, b) {
        ('R', 'P') | ('P', 'S') | ('S', 'R') => 0,
        ('R', 'R') | ('P', 'P') | ('S', 'S') => 1,
        ('R', 'S') | ('P', 'R') | ('S', 'P') => 2,
        _ => 0,
    }
}
