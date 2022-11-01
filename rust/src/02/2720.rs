use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut cents: i32 = buf.trim().parse().unwrap();

        let mut counts = [0, 0, 0, 0];
        let charges = [25, 10, 5, 1];

        for (i, charge) in counts.iter_mut().enumerate() {
            *charge += cents / charges[i];
            cents %= charges[i];
        }

        if let [quarter, dime, nickel, penny] = counts[..] {
            writeln!(stdout, "{quarter} {dime} {nickel} {penny}").unwrap();
        }
    }
}
