use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let first_prize = [500, 300, 200, 50, 30, 10];
    let first_ranks = [1, 3, 6, 10, 15, 21];

    let second_prize = [512, 256, 128, 64, 32];
    let second_ranks = [1, 3, 7, 15, 31];

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut total_prize = 0;

        if let [a, b] = parse_int_vec(&buf)[..] {
            for (i, &rank) in first_ranks.iter().enumerate() {
                if a > 0 && a <= rank {
                    total_prize += first_prize[i];
                    break;
                }
            }

            for (i, &rank) in second_ranks.iter().enumerate() {
                if b > 0 && b <= rank {
                    total_prize += second_prize[i];
                    break;
                }
            }
        }

        writeln!(stdout, "{}", total_prize * 10000).unwrap();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
