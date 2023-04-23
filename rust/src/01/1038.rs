fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = -1;

    for i in 1..=10 {
        let (num, is_finished) = permutations(0, &mut vec![0; i], &mut count, n);

        if is_finished {
            println!("{num}");
            return;
        }
    }

    println!("-1");
}

fn permutations(depth: usize, selected: &mut Vec<u8>, count: &mut i32, n: i32) -> (i64, bool) {
    if depth == selected.len() {
        *count += 1;
        // println!("{selected:?}");
        return if *count == n {
            let num = selected
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as i64);

            (num, true)
        } else {
            (Default::default(), false)
        };
    }

    for digit in 0..=9 {
        if depth > 0 && selected[depth - 1] <= digit {
            continue;
        }

        selected[depth] = digit;
        let (num, is_finished) = permutations(depth + 1, selected, count, n);

        if is_finished {
            return (num, true);
        }
    }

    (Default::default(), false)
}
