fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim();
    let most_ahead = word.chars().min().unwrap();

    let mut split_reversed: Vec<String> = (1..=word.len() - 2)
        .filter_map(|i| {
            let (first, rest) = word.split_at(i);

            (first.chars().last() == Some(most_ahead)).then(|| {
                (1..=rest.len() - 1).map(move |j| {
                    let (second, third) = rest.split_at(j);
                    [first, second, third]
                })
            })
        })
        .flatten()
        .map(|splitted| {
            splitted
                .map(|token| token.chars().rev().collect::<String>())
                .into_iter()
                .collect()
        })
        .collect();

    println!("{}", split_reversed.select_nth_unstable(0).1);
}
