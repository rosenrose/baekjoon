use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let letters = input
            .to_lowercase()
            .chars()
            .fold([false; 26], |mut acc, c| {
                if !c.is_alphabetic() {
                    return acc;
                }

                acc[c as usize - 'a' as usize] = true;
                acc
            });

        if letters.iter().all(|&b| b) {
            println!("pangram");
            continue;
        }

        println!(
            "missing {}",
            letters
                .iter()
                .enumerate()
                .filter_map(|(ch, &b)| (!b).then(|| (ch as u8 + 'a' as u8) as char))
                .collect::<String>()
        );
    }
}
