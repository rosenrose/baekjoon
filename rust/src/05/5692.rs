use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "0") {
        let num: i32 = input
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, ch)| (ch - b'0') as i32 * (1..=i as i32 + 1).product::<i32>())
            .sum();

        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}
