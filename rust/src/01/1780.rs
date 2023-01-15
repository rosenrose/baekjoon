use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

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
    let (mut count_m, mut count_0, mut count_p) = (0, 0, 0);

    'outer: for i in y..y + n {
        for j in x..x + n {
            match paper[i][j] {
                -1 => count_m += 1,
                0 => count_0 += 1,
                1 => count_p += 1,
                _ => (),
            }

            match (count_m > 0, count_0 > 0, count_p > 0) {
                (true, true, _) | (true, _, true) | (_, true, true) => break 'outer,
                _ => (),
            }
        }
    }

    if count_m == n * n {
        *minus += 1;
        return;
    }
    if count_0 == n * n {
        *zero += 1;
        return;
    }
    if count_p == n * n {
        *plus += 1;
        return;
    }

    let unit = n / 3;

    cut(paper, x, y, unit, minus, zero, plus);
    cut(paper, x + unit, y, unit, minus, zero, plus);
    cut(paper, x + unit * 2, y, unit, minus, zero, plus);
    cut(paper, x, y + unit, unit, minus, zero, plus);
    cut(paper, x + unit, y + unit, unit, minus, zero, plus);
    cut(paper, x + unit * 2, y + unit, unit, minus, zero, plus);
    cut(paper, x, y + unit * 2, unit, minus, zero, plus);
    cut(paper, x + unit, y + unit * 2, unit, minus, zero, plus);
    cut(paper, x + unit * 2, y + unit * 2, unit, minus, zero, plus);
}
