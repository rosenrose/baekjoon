fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim();
    let most_ahead = word.chars().min().unwrap();

    let mut split_reversed: Vec<String> = (1..=word.len() - 2)
        .map(|i| word.split_at(i))
        .filter(|&(first, _)| first.chars().last().unwrap() == most_ahead)
        .map(|(first, rest)| {
            (1..=rest.len() - 1).map(move |j| {
                let (second, third) = rest.split_at(j);
                [first, second, third]
            })
        })
        .flatten()
        .map(|splitted| {
            let reversed = splitted.map(|piece| piece.chars().rev().collect::<String>());

            String::from_iter(reversed)
        })
        .collect();

    split_reversed.sort();

    println!("{}", split_reversed[0]);
}
