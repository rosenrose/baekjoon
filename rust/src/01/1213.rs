use std::collections::BTreeMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut counts = BTreeMap::new();

    buf.trim().chars().for_each(|ch| {
        counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    });

    let odds: Vec<_> = counts
        .iter()
        .filter_map(|(ch, &count)| (count % 2 == 1).then(|| ch))
        .collect();

    if odds.len() > 1 {
        println!("I'm Sorry Hansoo");
        return;
    }

    let mut palindrome = match odds.get(0) {
        Some(c) => c.to_string(),
        None => String::new(),
    };

    for (&ch, &count) in counts.iter().rev() {
        for _ in 0..count / 2 {
            palindrome.insert(0, ch);
            palindrome.push(ch);
        }
    }

    println!("{palindrome}");
}
