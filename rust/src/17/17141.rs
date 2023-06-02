#[derive(Clone)]
enum Cells {
    Empty,
    Wall,
    Virus,
}

use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut viruse_places = Vec::new();
    let mut empty_cells = 0;

    let room: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..n)
                .map(|c| match input() {
                    0 => {
                        empty_cells += 1;
                        Cells::Empty
                    }
                    1 => Cells::Wall,
                    2 => {
                        viruse_places.push((r, c));
                        empty_cells += 1;
                        Cells::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let min_time = combinations(0, 0, &mut vec![0; m], &viruse_places, &room, empty_cells);

    println!("{}", min_time.unwrap_or(-1));
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    viruse_places: &[(usize, usize)],
    room: &[Vec<Cells>],
    empty_cells: i32,
) -> Option<i32> {
    if depth == selected.len() {
        return simulate(selected, viruse_places, room.to_owned(), empty_cells);
    }

    let takes = viruse_places.len() - (selected.len() - 1);

    (start..depth + takes)
        .flat_map(|i| {
            selected[depth] = i;
            combinations(depth + 1, i + 1, selected, viruse_places, room, empty_cells)
        })
        .min()
}

fn simulate(
    selected: &[usize],
    viruse_places: &[(usize, usize)],
    mut room: Vec<Vec<Cells>>,
    mut empty_cells: i32,
) -> Option<i32> {
    let size = room.len();
    let mut max_time = 0;
    let mut queue = VecDeque::with_capacity(selected.len());

    for (virus_r, virus_c) in selected.iter().map(|&i| viruse_places[i]) {
        room[virus_r][virus_c] = Cells::Virus;
        empty_cells -= 1;
        queue.push_back(((virus_r, virus_c), 0));
    }

    while let Some(((r, c), time)) = queue.pop_front() {
        max_time = time.max(max_time);

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(size - 1), c),
            (r, (c + 1).min(size - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if !matches!(room[adj_r][adj_c], Cells::Empty) {
                continue;
            }

            room[adj_r][adj_c] = Cells::Virus;
            empty_cells -= 1;
            queue.push_back(((adj_r, adj_c), time + 1));
        }
    }

    (empty_cells == 0).then_some(max_time)
}
