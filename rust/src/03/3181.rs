fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ignore = [
        "i", "pa", "te", "ni", "niti", "a", "ali", "nego", "no", "ili",
    ];

    let short: String = buf
        .split_whitespace()
        .enumerate()
        .filter_map(|(i, word)| {
            (i == 0 || !ignore.contains(&word))
                .then_some(word.to_uppercase().chars().nth(0).unwrap())
        })
        .collect();

    println!("{short}")
}
