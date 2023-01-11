fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut count = 0;

    n_queen(n, 0, &mut vec![0; n], &mut count);

    println!("{count}");
}

fn n_queen(n: usize, row: usize, selected: &mut Vec<usize>, count: &mut i32) {
    if row == n {
        *count += 1;
        return;
    }

    for col in 0..n {
        if selected
            .iter()
            .take(row)
            .enumerate()
            .any(|(r, &c)| c == col || c.abs_diff(col) == r.abs_diff(row))
        {
            continue;
        }

        selected[row] = col;

        n_queen(n, row + 1, selected, count);
    }
}
