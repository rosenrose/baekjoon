use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let tab: i32 = buf.trim().parse().unwrap();

        count += tab - 1;
    }

    count += 1;

    writeln!(stdout, "{count}").unwrap();
}
