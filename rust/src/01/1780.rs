use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let paper: Vec<_> = lines.map(|line| parse_int_vec(line)).collect();
    let (mut minus, mut zero, mut plus) = (0, 0, 0);

    cut(&paper, 0, 0, n, &mut minus, &mut zero, &mut plus);

    writeln!(stdout, "{minus}\n{zero}\n{plus}").unwrap();
}

fn cut(
    paper: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    n: usize,
    minus: &mut i32,
    zero: &mut i32,
    plus: &mut i32,
) {
    let (mut count_m1, mut count_0, mut count_p1) = (0, 0, 0);

    'outer: for i in y..y + n {
        for j in x..x + n {
            match paper[i][j] {
                -1 => count_m1 += 1,
                0 => count_0 += 1,
                1 => count_p1 += 1,
                _ => (),
            }

            match (count_m1 > 0, count_0 > 0, count_p1 > 0) {
                (true, true, _) | (true, _, true) | (_, true, true) => break 'outer,
                _ => (),
            }
        }
    }

    if count_m1 == n * n {
        *minus += 1;

        return;
    }
    if count_0 == n * n {
        *zero += 1;

        return;
    }
    if count_p1 == n * n {
        *plus += 1;

        return;
    }

    cut(paper, x, y, n / 3, minus, zero, plus);
    cut(paper, x + n / 3, y, n / 3, minus, zero, plus);
    cut(paper, x + n / 3 * 2, y, n / 3, minus, zero, plus);
    cut(paper, x, y + n / 3, n / 3, minus, zero, plus);
    cut(paper, x + n / 3, y + n / 3, n / 3, minus, zero, plus);
    cut(paper, x + n / 3 * 2, y + n / 3, n / 3, minus, zero, plus);
    cut(paper, x, y + n / 3 * 2, n / 3, minus, zero, plus);
    cut(paper, x + n / 3, y + n / 3 * 2, n / 3, minus, zero, plus);
    #[rustfmt::skip]
    cut(paper, x + n / 3 * 2, y + n / 3 * 2, n / 3, minus, zero, plus);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
