fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    let (a_cross, b_cross) = a
        .char_indices()
        .find_map(|(a_idx, ch)| b.contains(ch).then(|| (a_idx, b.find(ch).unwrap())))
        .unwrap();

    b.char_indices().for_each(|(i, ch)| {
        if i == b_cross {
            println!("{a}");
            return;
        }

        println!(
            "{}{ch}{}",
            ".".repeat(a_cross),
            ".".repeat(a.len() - a_cross - 1)
        );
    });
}
