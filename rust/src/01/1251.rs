fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim();
    let most_ahead_ch = word.chars().min().unwrap();

    let most_ahead_word = (1..word.len() - 1)
        .filter_map(|i| {
            let (first, rest) = word.split_at(i);

            (first.chars().last() == Some(most_ahead_ch)).then(|| {
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
                .join("")
        })
        .min()
        .unwrap();

    println!("{most_ahead_word}");
}
