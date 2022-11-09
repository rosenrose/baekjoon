use std::cmp::Ordering;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(&buf);

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let games = parse_int(&buf);

        let (a, b) = (0..games)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();

                match parse_str_vec(&buf)[..] {
                    ["R", "P"] => (0, 1),
                    ["R", "S"] => (1, 0),
                    ["P", "R"] => (1, 0),
                    ["P", "S"] => (0, 1),
                    ["S", "R"] => (0, 1),
                    ["S", "P"] => (1, 0),
                    _ => (0, 0),
                }
            })
            .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
            .unwrap();

        let result = match a.cmp(&b) {
            Ordering::Greater => "Player 1",
            Ordering::Equal => "TIE",
            Ordering::Less => "Player 2",
        };

        writeln!(stdout, "{result}").unwrap();
    }
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
