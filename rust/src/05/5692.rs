use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "0") {
        let num: i32 = input
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| (c as u8 - '0' as u8) as i32 * (1..=i as i32 + 1).product::<i32>())
            .sum();

        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}
