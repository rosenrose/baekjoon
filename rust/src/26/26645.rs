fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let level_number: Vec<_> = [(209, 8), (219, 4), (229, 2), (239, 1)]
        .iter()
        .enumerate()
        .filter_map(|(i, &(limit, count))| {
            (n <= limit).then(|| ((limit + 1).min(n + count), i + 1))
        })
        .collect();

    let (_, number) = level_number.iter().max().unwrap();

    println!("{number}");
}
