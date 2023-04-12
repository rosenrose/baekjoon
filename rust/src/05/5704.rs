use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "*") {
        let mut letters = [false; 26];

        for ch in input.chars() {
            if ch == ' ' {
                continue;
            }

            letters[ch as usize - 'a' as usize] = true;
        }

        writeln!(
            output,
            "{}",
            if letters.iter().all(|&b| b) { 'Y' } else { 'N' }
        )
        .unwrap();
    }

    print!("{output}");
}
