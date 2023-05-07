#[derive(Clone)]
enum Cells {
    Empty,
    CCTV,
    Wall,
    Watching,
}

enum Dirs {
    Up,
    Right,
    Down,
    Left,
}

use std::io;
use Dirs::*;

const CCTV_TYPES: [&[&[Dirs]]; 5] = [
    &[&[Up], &[Right], &[Down], &[Left]],
    &[&[Left, Right], &[Up, Down]],
    &[&[Up, Right], &[Right, Down], &[Down, Left], &[Left, Up]],
    &[
        &[Left, Up, Right],
        &[Up, Right, Down],
        &[Right, Down, Left],
        &[Down, Left, Up],
    ],
    &[&[Up, Right, Down, Left]],
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut cctvs = Vec::new();
    let mut empty_cell_count = 0;

    let room: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..m)
                .map(|c| match input() {
                    0 => {
                        empty_cell_count += 1;
                        Cells::Empty
                    }
                    cell @ 1..=5 => {
                        cctvs.push(((r as usize, c as usize), cell));
                        Cells::CCTV
                    }
                    6 => Cells::Wall,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let min_blind_spots = product(
        0,
        &mut vec![0; cctvs.len()],
        &room,
        &cctvs,
        &(n, m),
        &empty_cell_count,
    );

    println!("{min_blind_spots}");
}

fn product(
    depth: usize,
    selected: &mut Vec<usize>,
    room: &[Vec<Cells>],
    cctvs: &[((usize, usize), usize)],
    size: &(usize, usize),
    empty_cell_count: &i32,
) -> i32 {
    if depth == selected.len() {
        return simulate(selected, room.to_owned(), cctvs, size, *empty_cell_count);
    }

    let (_, cctv_num) = cctvs[depth];

    (0..CCTV_TYPES[cctv_num - 1].len())
        .map(|type_| {
            selected[depth] = type_;

            product(depth + 1, selected, room, cctvs, size, empty_cell_count)
        })
        .min()
        .unwrap()
}

fn simulate(
    selected: &[usize],
    mut room: Vec<Vec<Cells>>,
    cctvs: &[((usize, usize), usize)],
    &(row_len, col_len): &(usize, usize),
    mut empty_cell_count: i32,
) -> i32 {
    let mut watch_stop_if_wall = |r: usize, c: usize| {
        match room[r][c] {
            Cells::Wall => return true,
            Cells::Empty => {
                room[r][c] = Cells::Watching;
                empty_cell_count -= 1;
            }
            _ => (),
        }

        false
    };

    for (i, &((row, col), num)) in cctvs.iter().enumerate() {
        for dir in CCTV_TYPES[num - 1][selected[i]].iter() {
            match dir {
                Up => {
                    for c in (0..col).rev() {
                        if watch_stop_if_wall(row, c) {
                            break;
                        }
                    }
                }
                Down => {
                    for c in col + 1..col_len {
                        if watch_stop_if_wall(row, c) {
                            break;
                        }
                    }
                }
                Left => {
                    for r in (0..row).rev() {
                        if watch_stop_if_wall(r, col) {
                            break;
                        }
                    }
                }
                Right => {
                    for r in row + 1..row_len {
                        if watch_stop_if_wall(r, col) {
                            break;
                        }
                    }
                }
            }
        }
    }

    empty_cell_count
}
