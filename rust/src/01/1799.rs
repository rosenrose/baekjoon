use std::io;

const MAX: usize = 100 / 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let (mut white_cells, mut black_cells) = ([(0, 0); MAX], [(0, 0); MAX]);
    let (mut white_cells_len, mut black_cells_len) = (0, 0);

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            if num == 0 {
                continue;
            }

            if matches!((r & 1, c & 1), (0, 0) | (1, 1)) {
                white_cells[white_cells_len] = (r, c);
                white_cells_len += 1;
            } else {
                black_cells[black_cells_len] = (r, c);
                black_cells_len += 1;
            }
        }
    }

    let white_count = product(0, &mut [false; MAX], white_cells_len, &white_cells, 0);
    let black_count = product(0, &mut [false; MAX], black_cells_len, &black_cells, 0);

    println!("{}", white_count + black_count);
}

fn product(
    depth: usize,
    selected: &mut [bool; MAX],
    selected_len: usize,
    cells: &[(usize, usize)],
    count: i32,
) -> i32 {
    if depth == selected_len {
        return count;
    }

    let (row, col) = cells[depth];
    let mut result_on = 0;

    if (0..depth)
        .filter_map(|i| selected[i].then_some(cells[i]))
        .all(|(r, c)| r.abs_diff(row) != c.abs_diff(col))
    {
        selected[depth] = true;
        result_on = product(depth + 1, selected, selected_len, cells, count + 1);
        selected[depth] = false;
    }

    let result_off = product(depth + 1, selected, selected_len, cells, count);

    result_on.max(result_off)
}
