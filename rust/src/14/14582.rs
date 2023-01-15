use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let team1: Vec<_> = (0..9).map(|_| input.next().unwrap()).collect();
    let team2: Vec<_> = (0..9).map(|_| input.next().unwrap()).collect();

    let mut was_winning = false;
    let (mut score1, mut score2) = (0, 0);

    for i in 0..9 {
        score1 += team1[i];

        if !was_winning && (score1 > score2) {
            was_winning = true;
        }

        score2 += team2[i];
    }

    println!("{}", if was_winning { "Yes" } else { "No" });
}
