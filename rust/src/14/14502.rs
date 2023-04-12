#[derive(Default, Clone, PartialEq)]
enum Cells {
    #[default]
    Empty,
    Wall,
    Virus,
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut empty_cells = Vec::new();
    let mut viruses = Vec::new();

    let room: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..m)
                .map(|c| match input() {
                    0 => {
                        empty_cells.push((r, c));
                        Cells::Empty
                    }
                    1 => Cells::Wall,
                    2 => {
                        viruses.push((r, c));
                        Cells::Virus
                    }
                    _ => Default::default(),
                })
                .collect()
        })
        .collect();

    let empty_cells_len = empty_cells.len();
    let mut max_safe_area = 0;

    for a in 0..empty_cells_len - 2 {
        for b in a + 1..empty_cells_len - 1 {
            for c in b + 1..empty_cells_len {
                let safe_area = simulate(
                    (empty_cells[a], empty_cells[b], empty_cells[c]),
                    room.clone(),
                    viruses.clone(),
                    (n, m),
                    empty_cells_len,
                );

                max_safe_area = safe_area.max(max_safe_area);
            }
        }
    }

    println!("{max_safe_area}");
}

fn simulate(
    (new_wall1, new_wall2, new_wall3): ((usize, usize), (usize, usize), (usize, usize)),
    mut room: Vec<Vec<Cells>>,
    mut stack: Vec<(usize, usize)>,
    (row_len, col_len): (usize, usize),
    mut safe_area: usize,
) -> usize {
    room[new_wall1.0][new_wall1.1] = Cells::Wall;
    room[new_wall2.0][new_wall2.1] = Cells::Wall;
    room[new_wall3.0][new_wall3.1] = Cells::Wall;
    safe_area -= 3;

    while let Some((r, c)) = stack.pop() {
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(row_len - 1), c),
            (r, (c + 1).min(col_len - 1)),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
            if room[adj_r][adj_c] == Cells::Empty {
                room[adj_r][adj_c] = Cells::Virus;
                safe_area -= 1;

                stack.push((adj_r, adj_c));
            }
        }
    }

    safe_area
}
