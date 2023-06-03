fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim().as_bytes();
    let offset = b'A';
    let mut counts = [0; 26];

    for ch in input {
        counts[(ch - offset) as usize] += 1;
    }

    let odds: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(ch, count)| (count % 2 == 1).then_some((ch as u8 + offset) as char))
        .collect();

    if odds.len() > 1 {
        println!("I'm Sorry Hansoo");
        return;
    }

    let mut palindrome = if let Some(ch) = odds.get(0) {
        ch.to_string()
    } else {
        String::new()
    };

    for (ch, &count) in counts.iter().enumerate().rev() {
        let ch = (ch as u8 + offset) as char;

        for _ in 0..count / 2 {
            palindrome.insert(0, ch);
            palindrome.push(ch);
        }
    }

    println!("{palindrome}");
}
