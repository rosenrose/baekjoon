use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.split_whitespace().map(parse_int).next().unwrap();
    let arr: Vec<Vec<i32>> = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();

            parse_int_vec(&buf)
        })
        .collect();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let k = parse_int(buf.trim());

    for _ in 0..k {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [i, j, x, y] = parse_int_vec(&buf)[..] {
            let (i, j, x, y) = (
                i as usize - 1,
                j as usize - 1,
                x as usize - 1,
                y as usize - 1,
            );

            let sum: i32 = arr[i..=x]
                .iter()
                .map(|row| row[j..=y].iter().sum::<i32>())
                .sum();

            writeln!(stdout, "{sum}").unwrap();
        }
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(parse_int).collect()
}
