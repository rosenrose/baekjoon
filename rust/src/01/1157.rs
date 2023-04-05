fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim().to_uppercase();
    let mut counts = [0; 26];
    let mut max_count = 1;

    for ch in word.chars() {
        let index = ch as usize - 'A' as usize;

        counts[index] += 1;
        max_count = max_count.max(counts[index]);
    }

    let mut most_letters = counts.iter().enumerate().filter(|(_, &c)| c == max_count);
    let (most_letter, _) = most_letters.next().unwrap();

    if most_letters.next().is_some() {
        println!("?");
    } else {
        println!("{}", (most_letter as u8 + 'A' as u8) as char);
    }
}
