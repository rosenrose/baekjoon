use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let cards: Vec<_> = (0..n)
        .map(|_| {
            let mut card: Vec<_> = input.by_ref().take(m).collect();
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
            .filter_map(|(p, row)| (row[col] == max_card).then_some(p))
            .for_each(|player| {
                scores[player] += 1;
                max_score = scores[player].max(max_score);
            });
    }

    for (player, _) in scores.iter().enumerate().filter(|(_, &s)| s == max_score) {
        print!("{} ", player + 1);
    }
}
