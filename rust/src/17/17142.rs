#[derive(Copy, Clone)]
enum Cells {
    Empty,
    Wall,
    VirusActive,
    VirusInactive,
}

use std::collections::VecDeque;
use std::io;

const SIZE_MAX: usize = 50;
const VIRUS_MAX: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut virus_places = [(0, 0); SIZE_MAX * SIZE_MAX];
    let mut virus_places_len = 0;
    let mut empty_cell_count = 0;
    let mut room = [[Cells::Empty; SIZE_MAX]; SIZE_MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            room[r][c] = match num {
                0 => {
                    empty_cell_count += 1;
                    Cells::Empty
                }
                1 => Cells::Wall,
                2 => {
                    virus_places[virus_places_len] = (r, c);
                    virus_places_len += 1;
                    Cells::VirusInactive
                }
                _ => unreachable!(),
            };
        }
    }

    let min_time = combinations(
        0,
        0,
        &mut [0; VIRUS_MAX][..m],
        &virus_places[..virus_places_len],
        &room[..n],
        empty_cell_count,
    );

    println!("{}", min_time.unwrap_or(-1));
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize],
    virus_places: &[(usize, usize)],
    room: &[[Cells; SIZE_MAX]],
    empty_cell_count: i32,
) -> Option<i32> {
    let size = room.len();

    if depth == selected.len() {
        let mut temp = [[Cells::Empty; SIZE_MAX]; SIZE_MAX];

        for (r, row) in room.iter().enumerate() {
            temp[r][..size].copy_from_slice(&row[..size]);
        }

        return simulate(selected, virus_places, &mut temp[..size], empty_cell_count);
    }

    let takes = virus_places.len() - (selected.len() - 1);

    (start..depth + takes)
        .filter_map(|i| {
            selected[depth] = i;
            combinations(
                depth + 1,
                i + 1,
                selected,
                virus_places,
                room,
                empty_cell_count,
            )
        })
        .min()
}

fn simulate(
    selected: &[usize],
    virus_places: &[(usize, usize)],
    room: &mut [[Cells; SIZE_MAX]],
    mut empty_cell_count: i32,
) -> Option<i32> {
    let size = room.len();
    let mut max_time = 0;
    let mut queue = VecDeque::with_capacity(selected.len());

    for (virus_r, virus_c) in selected.iter().map(|&i| virus_places[i]) {
        room[virus_r][virus_c] = Cells::VirusActive;
        queue.push_back(((virus_r, virus_c), 0));
    }

    while let Some(((r, c), time)) = queue.pop_front() {
        if empty_cell_count == 0 {
            break;
        }

        let next_time = time + 1;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(size - 1), c),
            (r, (c + 1).min(size - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if matches!(room[adj_r][adj_c], Cells::Wall | Cells::VirusActive) {
                continue;
            }

            if let Cells::Empty = room[adj_r][adj_c] {
                empty_cell_count -= 1;
            }

            room[adj_r][adj_c] = Cells::VirusActive;
            max_time = next_time.max(max_time);
            queue.push_back(((adj_r, adj_c), next_time));
        }
    }

    (empty_cell_count == 0).then_some(max_time)
}
