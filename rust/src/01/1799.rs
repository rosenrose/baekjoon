use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let (mut empty_cells_white, mut empty_cells_black) = (Vec::new(), Vec::new());

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            if num == 0 {
                continue;
            }

            if matches!((r & 1, c & 1), (0, 0) | (1, 1)) {
                empty_cells_white.push((r, c));
            } else {
                empty_cells_black.push((r, c));
            }
        }
    }

    let white_count = product(
        0,
        &mut vec![false; empty_cells_white.len()],
        &empty_cells_white,
        0,
    );
    let black_count = product(
        0,
        &mut vec![false; empty_cells_black.len()],
        &empty_cells_black,
        0,
    );

    println!("{}", white_count + black_count);
}

fn product(
    depth: usize,
    selected: &mut Vec<bool>,
    empty_cells: &[(usize, usize)],
    count: i32,
) -> i32 {
    if depth == selected.len() {
        return count;
    }

    let (row, col) = empty_cells[depth];
    let mut result_on = 0;

    if (0..depth)
        .filter_map(|i| selected[i].then_some(empty_cells[i]))
        .all(|(r, c)| r.abs_diff(row) != c.abs_diff(col))
    {
        selected[depth] = true;
        result_on = product(depth + 1, selected, empty_cells, count + 1);
        selected[depth] = false;
    }

    let result_off = product(depth + 1, selected, empty_cells, count);

    result_on.max(result_off)
}
