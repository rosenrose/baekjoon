fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let words = buf.split_whitespace();
    let ignore = [
        "i", "pa", "te", "ni", "niti", "a", "ali", "nego", "no", "ili",
    ];

    let short: String = words
        .enumerate()
        .filter_map(|(i, word)| {
            (i == 0 || !ignore.contains(&word)).then(|| word.to_uppercase().chars().nth(0).unwrap())
        })
        .collect();

    println!("{short}")
}
