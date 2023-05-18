use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "*") {
        let mut chars = [false; 26];

        for ch in input.as_bytes().iter().filter(|&&ch| ch != b' ') {
            chars[(ch - b'a') as usize] = true;
        }

        writeln!(
            output,
            "{}",
            if chars.iter().all(|&b| b) { 'Y' } else { 'N' }
        )
        .unwrap();
    }

    print!("{output}");
}
