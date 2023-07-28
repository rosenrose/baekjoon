#[derive(Clone)]
enum Cells {
    Empty,
    Wall,
    Virus,
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let mut empty_cells = Vec::new();
    let mut viruses = Vec::new();

    let room: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width)
                .enumerate()
                .map(|(c, num)| match num {
                    0 => {
                        empty_cells.push((r, c));
                        Cells::Empty
                    }
                    1 => Cells::Wall,
                    2 => {
                        viruses.push((r, c));
                        Cells::Virus
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let empty_cells_count = empty_cells.len();
    let mut max_safe_area = 0;

    for a in 0..empty_cells_count - 2 {
        for b in a + 1..empty_cells_count - 1 {
            for c in b + 1..empty_cells_count {
                let safe_area = simulate(
                    (empty_cells[a], empty_cells[b], empty_cells[c]),
                    room.clone(),
                    viruses.clone(),
                    empty_cells_count,
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
            ((r + 1).min(room.len() - 1), c),
            (r, (c + 1).min(room[0].len() - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if !matches!(room[adj_r][adj_c], Cells::Empty) {
                continue;
            }

            room[adj_r][adj_c] = Cells::Virus;
            safe_area -= 1;

            stack.push((adj_r, adj_c));
        }
    }

    safe_area
}
