use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    while let [Some(a), Some(b)] = [(); 2].map(|_| input.next()) {
        writeln!(output, "{}", a + b).unwrap();
    }

    print!("{output}");
}
