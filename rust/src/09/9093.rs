use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let words = buf.split_whitespace();
        let reversed: Vec<String> = words.map(|word| word.chars().rev().collect()).collect();

        writeln!(stdout, "{}", reversed.join(" ")).unwrap();
    }
}
