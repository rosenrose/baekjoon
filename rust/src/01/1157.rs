fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim().to_uppercase();
    let mut letter_counts = [0; 26];
    let mut max_count = 1;

    for c in word.chars() {
        let index = c as usize - 'A' as usize;

        letter_counts[index] += 1;
        max_count = max_count.max(letter_counts[index]);
    }

    let mut most_letters = letter_counts
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == max_count);
    let (most_letter, _) = most_letters.next().unwrap();

    match most_letters.next() {
        Some(_) => println!("?"),
        None => println!("{}", (most_letter as u8 + 'A' as u8) as char),
    }
}
