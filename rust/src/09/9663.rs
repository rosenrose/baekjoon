fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let count = n_queen(0, n, &mut vec![0; n]);

    println!("{count}");
}

fn n_queen(row: usize, n: usize, selected: &mut Vec<usize>) -> i32 {
    if row == n {
        return 1;
    }

    (0..n)
        .map(|col| {
            if selected[..row]
                .iter()
                .enumerate()
                .any(|(r, &c)| c == col || c.abs_diff(col) == r.abs_diff(row))
            {
                return 0;
            }

            selected[row] = col;
            n_queen(row + 1, n, selected)
        })
        .sum()
}
