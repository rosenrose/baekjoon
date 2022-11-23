fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim();
    let most_ahead = word.chars().min().unwrap();

    let mut split_reversed: Vec<String> = (1..=word.len() - 2)
        .filter_map(|i| {
            let (first, rest) = word.split_at(i);

            if first.chars().last().unwrap() == most_ahead {
                Some((1..=rest.len() - 1).map(move |j| {
                    let (second, third) = rest.split_at(j);
                    [first, second, third]
                }))
            } else {
                None
            }
        })
        .flatten()
        .map(|splitted| {
            splitted
                .map(|token| token.chars().rev().collect::<String>())
                .into_iter()
                .collect()
        })
        .collect();

    split_reversed.sort();

    println!("{}", split_reversed[0]);
}
