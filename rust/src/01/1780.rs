use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut paper = vec![vec![0; n]; n];

    for r in 0..n {
        for c in 0..n {
            paper[r][c] = input.next().unwrap();
        }
    }

    let [mut minus, mut zero, mut plus] = [0; 3];

    cut(&paper, 0, 0, n, &mut minus, &mut zero, &mut plus);

    println!("{minus}\n{zero}\n{plus}");
}

fn cut(
    paper: &[Vec<i32>],
    x: usize,
    y: usize,
    n: usize,
    minus: &mut i32,
    zero: &mut i32,
    plus: &mut i32,
) {
    let [mut count_m, mut count_z, mut count_p] = [0; 3];

    'outer: for i in y..y + n {
        for j in x..x + n {
            match paper[i][j] {
                -1 => count_m += 1,
                0 => count_z += 1,
                1 => count_p += 1,
                _ => unreachable!(),
            }

            if matches!(
                (count_m > 0, count_z > 0, count_p > 0),
                (true, true, _) | (true, _, true) | (_, true, true)
            ) {
                break 'outer;
            }
        }
    }

    if count_m == n * n {
        *minus += 1;
        return;
    }
    if count_z == n * n {
        *zero += 1;
        return;
    }
    if count_p == n * n {
        *plus += 1;
        return;
    }

    let one_third = n / 3;
    let two_third = one_third << 1;

    cut(paper, x, y, one_third, minus, zero, plus);
    cut(paper, x + one_third, y, one_third, minus, zero, plus);
    cut(paper, x + two_third, y, one_third, minus, zero, plus);

    cut(paper, x, y + one_third, one_third, minus, zero, plus);
    #[rustfmt::skip]
    cut(paper, x + one_third, y + one_third, one_third, minus, zero, plus);
    #[rustfmt::skip]
    cut(paper, x + two_third, y + one_third, one_third, minus, zero, plus);

    cut(paper, x, y + two_third, one_third, minus, zero, plus);
    #[rustfmt::skip]
    cut( paper, x + one_third, y + two_third, one_third, minus, zero, plus);
    #[rustfmt::skip]
    cut(paper, x + two_third, y + two_third, one_third, minus, zero, plus);
}
