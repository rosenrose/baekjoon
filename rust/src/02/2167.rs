use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.split_whitespace().map(parse_int).next().unwrap();
    let arr: Vec<Vec<_>> = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut row = vec![0];

            buf.split_whitespace().map(parse_int).for_each(|num| {
                row.push(row.last().unwrap() + num);
            });

            row
        })
        .collect();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let k = parse_int(buf.trim());

    for _ in 0..k {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [i, j, x, y] = parse_int_vec(&buf)[..] {
            let (i, j) = (i - 1, j - 1);

            let sum: i32 = arr[i..x].iter().map(|row| row[y] - row[j]).sum();

            writeln!(stdout, "{sum}").unwrap();
        }
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
