use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut coords: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut coord = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

            (coord.next().unwrap(), coord.next().unwrap())
        })
        .collect();

    coords.sort_by(
        |(x1, y1), (x2, y2)| {
            if x1 == x2 {
                y1.cmp(y2)
            } else {
                x1.cmp(x2)
            }
        },
    );

    for (x, y) in coords {
        writeln!(stdout, "{x} {y}").unwrap();
    }
}
