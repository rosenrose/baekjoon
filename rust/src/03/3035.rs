use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let [.., zr, zc] = [(); 4].map(|_| input.next().unwrap().parse::<usize>().unwrap());

    for row in input {
        for _ in 0..zr {
            for ch in row.chars() {
                write!(output, "{}", ch.to_string().repeat(zc)).unwrap();
            }
            writeln!(output, "").unwrap();
        }
    }

    print!("{output}");
}
