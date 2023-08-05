use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    while let [Some(a), Some(b), Some(c)] = [(); 3].map(|_| input.next()) {
        if [a, b, c] == [0; 3] {
            break;
        }

        if c - b == b - a {
            writeln!(output, "AP {}", c + (c - b))
        } else {
            writeln!(output, "GP {}", c * (c / b))
        }
        .unwrap()
    }

    print!("{output}");
}
