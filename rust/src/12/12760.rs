use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let cards: Vec<_> = (0..n)
        .map(|_| {
            let mut card: Vec<_> = (0..m).map(|_| input()).collect();
            card.sort();

            card
        })
        .collect();

    let mut scores = vec![0; n];
    let mut max_score = 0;

    for col in (0..m).rev() {
        let max_card = cards.iter().map(|row| row[col]).max().unwrap();

        cards
            .iter()
            .enumerate()
            .filter(|(_, row)| row[col] == max_card)
            .for_each(|(player, _)| {
                scores[player] += 1;
                max_score = scores[player].max(max_score);
            });
    }

    for (player, _) in scores.iter().enumerate().filter(|(_, &s)| s == max_score) {
        print!("{} ", player + 1);
    }
}
