use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let n = (input.len() as f64).sqrt() as usize;
        let table: Vec<_> = input.as_bytes().chunks(n).collect();

        for col in (0..n).rev() {
            for row in table.iter() {
                write!(output, "{}", row[col] as char).unwrap();
            }
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
