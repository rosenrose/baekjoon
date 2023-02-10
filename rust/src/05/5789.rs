use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(str::as_bytes);
    let mut output = String::new();

    for num in input.skip(1) {
        let half = num.len() / 2;

        writeln!(
            output,
            "{}",
            if num[half - 1] == num[half] {
                "Do-it"
            } else {
                "Do-it-Not"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
