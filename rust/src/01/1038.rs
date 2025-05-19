use std::io;

const MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    let mut count = -1;

    for i in 1..=MAX {
        let (num, is_finished) = permutations(0, &mut [0; MAX], i, &mut count, n);

        if is_finished {
            println!("{num}");
            return;
        }
    }

    println!("-1");
}

fn permutations(
    depth: usize,
    selected: &mut [u8],
    selected_len: usize,
    count: &mut i32,
    n: i32,
) -> (i64, bool) {
    if depth == selected_len {
        *count += 1;
        // println!("{selected:?}");
        return if *count == n {
            let num = selected[..selected_len]
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as i64);

            (num, true)
        } else {
            (0, false)
        };
    }

    for digit in 0..=9 {
        if depth > 0 && selected[depth - 1] <= digit {
            continue;
        }

        selected[depth] = digit;
        let (num, is_finished) = permutations(depth + 1, selected, selected_len, count, n);

        if is_finished {
            return (num, true);
        }
    }

    (0, false)
}
