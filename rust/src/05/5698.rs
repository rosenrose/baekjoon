use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().take_while(|&input| input != "*") {
        let input = input.to_lowercase();
        let letter = input.chars().nth(0);

        writeln!(
            output,
            "{}",
            if input.split(' ').all(|s| s.chars().nth(0) == letter) {
                'Y'
            } else {
                'N'
            }
        )
        .unwrap();
    }

    print!("{output}");
}
