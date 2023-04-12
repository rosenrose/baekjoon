use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for (i, input) in buf.lines().enumerate().skip(1) {
        let mut counts = [0; 26];

        for ch in input.to_lowercase().chars() {
            if ch.is_alphabetic() {
                counts[ch as usize - 'a' as usize] += 1;
            }
        }

        print!("Case {i}: ");

        if counts.iter().all(|&c| c >= 3) {
            println!("Triple pangram!!!");
            continue;
        }
        if counts.iter().all(|&c| c >= 2) {
            println!("Double pangram!!");
            continue;
        }
        if counts.iter().all(|&c| c >= 1) {
            println!("Pangram!");
            continue;
        }

        println!("Not a pangram");
    }
}
