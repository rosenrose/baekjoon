use std::io;

const PLAYERS_MAX: usize = 100;
const CARDS_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut cards = [[0; CARDS_MAX]; PLAYERS_MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(m).enumerate() {
            cards[r][c] = num;
        }

        cards[r][..m].sort();
    }

    let mut scores = [0; PLAYERS_MAX];
    let mut max_score = 0;

    for col in (0..m).rev() {
        let max_card = cards[..n].iter().map(|row| row[col]).max().unwrap();

        cards[..n]
            .iter()
            .enumerate()
            .filter_map(|(p, row)| (row[col] == max_card).then_some(p))
            .for_each(|player| {
                scores[player] += 1;
                max_score = scores[player].max(max_score);
            });
    }

    for (player, _) in scores[..n]
        .iter()
        .enumerate()
        .filter(|(_, &s)| s == max_score)
    {
        print!("{} ", player + 1);
    }
}
