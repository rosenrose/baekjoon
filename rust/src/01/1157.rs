use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim().to_uppercase();
    let mut letter_counts = HashMap::new();
    let mut max_count = 1;

    for ch in word.chars() {
        letter_counts
            .entry(ch)
            .and_modify(|c| {
                *c += 1;
                max_count = max_count.max(*c);
            })
            .or_insert(1);
    }

    let mut most_letters = letter_counts.iter().filter(|(_, &c)| c == max_count);
    let (most_letter, _) = most_letters.next().unwrap();

    match most_letters.next() {
        Some(_) => println!("?"),
        None => println!("{most_letter}"),
    }
}
