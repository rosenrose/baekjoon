use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let counts: HashMap<_, _> = input
        .chars()
        .map(|c| (c, input.matches(c).count()))
        .collect();

    let mut counts = Vec::from_iter(counts);
    counts.sort();

    let odds: Vec<_> = counts
        .iter()
        .filter_map(|(ch, count)| (count % 2 == 1).then(|| ch))
        .collect();

    if odds.len() > 1 {
        println!("I'm Sorry Hansoo");
        return;
    }

    let mut palindrome = match odds.get(0) {
        Some(c) => c.to_string(),
        None => String::new(),
    };

    for &(ch, count) in counts.iter().rev() {
        for _ in 0..count / 2 {
            palindrome.insert(0, ch);
            palindrome.push(ch);
        }
    }

    println!("{palindrome}");
}
