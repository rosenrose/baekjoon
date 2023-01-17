use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let n = (input.len() as f64).sqrt() as usize;
        let table: Vec<_> = input.as_bytes().chunks(n).collect();

        for j in (0..n).rev() {
            for i in 0..n {
                write!(output, "{}", table[i][j] as char).unwrap();
            }
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
