fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut counts = [0; 26];
    let offset = b'A';

    for ch in buf.trim().to_uppercase().as_bytes() {
        counts[(ch - offset) as usize] += 1;
    }

    let max_count = counts.iter().max().unwrap();
    let most_letters: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(ch, count)| (count == max_count).then_some((ch as u8 + offset) as char))
        .collect();

    if most_letters.len() > 1 {
        println!("?");
        return;
    }

    println!("{}", most_letters[0]);
}
