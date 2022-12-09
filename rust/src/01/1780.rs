use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap() as usize;
    let paper: Vec<_> = (0..n)
        .map(|_| (0..n).map(|_| input.next().unwrap()).collect())
        .collect();

    let (mut minus, mut zero, mut plus) = (0, 0, 0);

    cut(&paper, 0, 0, n, &mut minus, &mut zero, &mut plus);

    println!("{minus}\n{zero}\n{plus}");
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
