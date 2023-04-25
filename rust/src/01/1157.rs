fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut counts = [0; 26];

    for ch in buf.trim().to_uppercase().chars() {
        counts[ch as usize - 'A' as usize] += 1;
    }

    let max_count = counts.iter().max().unwrap();
    let most_letters: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(ch, count)| (count == max_count).then_some((ch as u8 + 'A' as u8) as char))
        .collect();

    if most_letters.len() > 1 {
        println!("?");
        return;
    }

    println!("{}", most_letters[0]);
}
