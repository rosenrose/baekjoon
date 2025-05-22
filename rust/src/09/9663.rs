use std::io;
const MAX: usize = 14;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let count = n_queen(0, &mut [0; MAX][..n]);

    println!("{count}");
}

fn n_queen(row: usize, selected: &mut [usize]) -> i32 {
    if row == selected.len() {
        return 1;
    }

    (0..selected.len())
        .map(|col| {
            if selected[..row]
                .iter()
                .enumerate()
                .any(|(r, &c)| c == col || c.abs_diff(col) == r.abs_diff(row))
            {
                return 0;
            }

            selected[row] = col;

            n_queen(row + 1, selected)
        })
        .sum()
}
