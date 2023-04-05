use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "*") {
        let letters = input.chars().fold([false; 26], |mut acc, ch| {
            if ch == ' ' {
                return acc;
            }

            acc[ch as usize - 'a' as usize] = true;
            acc
        });

        writeln!(
            output,
            "{}",
            if letters.iter().all(|&b| b) { 'Y' } else { 'N' }
        )
        .unwrap();
    }

    print!("{output}");
}
