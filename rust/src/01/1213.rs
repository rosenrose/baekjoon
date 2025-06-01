use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.trim().as_bytes();

    let offset = b'A';
    let mut counts = [0; 26];

    for ch in input {
        counts[(ch - offset) as usize] += 1;
    }

    let mut odd_letter = '\0';

    for ch in counts
        .iter()
        .enumerate()
        .filter_map(|(ch, count)| (count % 2 == 1).then_some((ch as u8 + offset) as char))
    {
        if odd_letter != '\0' {
            println!("I'm Sorry Hansoo");
            return;
        }

        odd_letter = ch;
    }

    let mut palindrome = if odd_letter == '\0' {
        String::new()
    } else {
        odd_letter.to_string()
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
