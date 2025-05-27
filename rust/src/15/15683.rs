#[derive(Copy, Clone)]
enum Cells {
    Empty,
    CCTV,
    Wall,
    Watching,
}

enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

use std::io;
use Dirs::*;

const WIDTH_MAX: usize = 8;
const HEIGHT_MAX: usize = 8;
const CCTV_MAX: usize = 8;
const CCTV_TYPES: [&[&[Dirs]]; 5] = [
    &[&[Up], &[Down], &[Left], &[Right]],
    &[&[Up, Down], &[Left, Right]],
    &[&[Up, Right], &[Right, Down], &[Down, Left], &[Left, Up]],
    &[
        &[Up, Right, Down],
        &[Right, Down, Left],
        &[Down, Left, Up],
        &[Left, Up, Right],
    ],
    &[&[Up, Down, Left, Right]],
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let mut cctvs = [((0, 0), 0); CCTV_MAX];
    let mut cctvs_count = 0;
    let mut empty_cell_count = 0;

    let mut room = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            room[r][c] = match num {
                0 => {
                    empty_cell_count += 1;
                    Cells::Empty
                }
                cell @ 1..=5 => {
                    cctvs[cctvs_count] = ((r, c), cell);
                    cctvs_count += 1;
                    Cells::CCTV
                }
                6 => Cells::Wall,
                _ => unreachable!(),
            };
        }
    }

    let min_blind_spots = product(
        0,
        &mut [0; CCTV_MAX][..cctvs_count],
        &room[..height],
        width,
        &cctvs[..cctvs_count],
        &empty_cell_count,
    );

    println!("{min_blind_spots}");
}

fn product(
    depth: usize,
    selected: &mut [usize],
    room: &[[Cells; WIDTH_MAX]],
    width: usize,
    cctvs: &[((usize, usize), usize)],
    empty_cell_count: &i32,
) -> i32 {
    let height = room.len();

    if depth == selected.len() {
        let mut temp = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

        for (r, row) in room.iter().enumerate() {
            temp[r][..width].copy_from_slice(&row[..width]);
        }

        return simulate(
            selected,
            &mut temp[..height],
            width,
            cctvs,
            *empty_cell_count,
        );
    }

    let (_, cctv_num) = cctvs[depth];

    (0..CCTV_TYPES[cctv_num - 1].len())
        .map(|type_| {
            selected[depth] = type_;

            product(depth + 1, selected, room, width, cctvs, empty_cell_count)
        })
        .min()
        .unwrap()
}

fn simulate(
    selected: &[usize],
    room: &mut [[Cells; WIDTH_MAX]],
    width: usize,
    cctvs: &[((usize, usize), usize)],
    mut empty_cell_count: i32,
) -> i32 {
    let height = room.len();
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
        for dir in CCTV_TYPES[num - 1][selected[i]] {
            match dir {
                Up => {
                    for c in (0..col).rev() {
                        if watch_stop_if_wall(row, c) {
                            break;
                        }
                    }
                }
                Down => {
                    for c in col + 1..width {
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
                    for r in row + 1..height {
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
