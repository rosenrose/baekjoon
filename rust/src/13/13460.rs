#[derive(Copy, Clone, PartialEq, Debug)]
enum Cells {
    Red,
    Blue,
    Empty,
    Wall,
    Hole,
}

#[derive(Copy, Clone)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::VecDeque;
use std::io;
use Cells::*;
use Dirs::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let (mut red, mut blue) = ((0, 0), (0, 0));
    let map: Vec<Vec<_>> = input
        .skip(2)
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '.' => Empty,
                    '#' => Wall,
                    'O' => Hole,
                    'R' => {
                        red = (r, c);
                        Empty
                    }
                    'B' => {
                        blue = (r, c);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    println!("{}", simulate(map, red, blue).unwrap_or(-1));
}

fn simulate(mut map: Vec<Vec<Cells>>, red: (usize, usize), blue: (usize, usize)) -> Option<i32> {
    let mut queue = VecDeque::from([((red, blue), 0)]);

    while let Some(((red, blue), time)) = queue.pop_front() {
        if time >= 10 {
            return None;
        }

        let next_time = time + 1;
        map[red.0][red.1] = Red;
        map[blue.0][blue.1] = Blue;

        for dir in [Up, Down, Left, Right] {
            let (moved_red, moved_blue) = match dir {
                Up => {
                    if red.0 < blue.0 {
                        move_red_then_blue(red, blue, dir, &mut map)
                    } else {
                        move_blue_then_red(red, blue, dir, &mut map)
                    }
                }
                Down => {
                    if red.0 < blue.0 {
                        move_blue_then_red(red, blue, dir, &mut map)
                    } else {
                        move_red_then_blue(red, blue, dir, &mut map)
                    }
                }
                Left => {
                    if red.1 < blue.1 {
                        move_red_then_blue(red, blue, dir, &mut map)
                    } else {
                        move_blue_then_red(red, blue, dir, &mut map)
                    }
                }
                Right => {
                    if red.1 < blue.1 {
                        move_blue_then_red(red, blue, dir, &mut map)
                    } else {
                        move_red_then_blue(red, blue, dir, &mut map)
                    }
                }
            };

            if (moved_red, moved_blue) == (red, blue) {
                continue;
            }
            if map[moved_blue.0][moved_blue.1] == Hole {
                continue;
            }

            if map[moved_red.0][moved_red.1] == Hole {
                return Some(next_time);
            }

            queue.push_back(((moved_red, moved_blue), next_time));
        }

        map[red.0][red.1] = Empty;
        map[blue.0][blue.1] = Empty;
    }

    None
}

fn move_red_then_blue(
    red: (usize, usize),
    blue: (usize, usize),
    dir: Dirs,
    map: &mut Vec<Vec<Cells>>,
) -> ((usize, usize), (usize, usize)) {
    let moved_red = get_moved_coord(red, dir, map);
    map[red.0][red.1] = Empty;

    let temp = map[moved_red.0][moved_red.1];

    if temp != Hole {
        map[moved_red.0][moved_red.1] = Red;
    }

    let moved_blue = get_moved_coord(blue, dir, map);

    map[red.0][red.1] = Red;
    map[moved_red.0][moved_red.1] = temp;

    (moved_red, moved_blue)
}

fn move_blue_then_red(
    red: (usize, usize),
    blue: (usize, usize),
    dir: Dirs,
    map: &mut Vec<Vec<Cells>>,
) -> ((usize, usize), (usize, usize)) {
    let moved_blue = get_moved_coord(blue, dir, map);
    map[blue.0][blue.1] = Empty;

    let temp = map[moved_blue.0][moved_blue.1];

    if temp != Hole {
        map[moved_blue.0][moved_blue.1] = Blue;
    }

    let moved_red = get_moved_coord(red, dir, map);

    map[blue.0][blue.1] = Blue;
    map[moved_blue.0][moved_blue.1] = temp;

    (moved_red, moved_blue)
}

fn get_moved_coord((r, c): (usize, usize), dir: Dirs, map: &[Vec<Cells>]) -> (usize, usize) {
    let (width, height) = (map[0].len(), map.len());

    match dir {
        Up => {
            let moved_r = (0..r)
                .rev()
                .find(|&moved_r| map[moved_r][c] != Empty)
                .unwrap();

            if map[moved_r][c] == Hole {
                (moved_r, c)
            } else {
                (moved_r + 1, c)
            }
        }
        Down => {
            let moved_r = (r + 1..height)
                .find(|&moved_r| map[moved_r][c] != Empty)
                .unwrap();

            if map[moved_r][c] == Hole {
                (moved_r, c)
            } else {
                (moved_r - 1, c)
            }
        }
        Left => {
            let moved_c = (0..c)
                .rev()
                .find(|&moved_c| map[r][moved_c] != Empty)
                .unwrap();

            if map[r][moved_c] == Hole {
                (r, moved_c)
            } else {
                (r, moved_c + 1)
            }
        }
        Right => {
            let moved_c = (c + 1..width)
                .find(|&moved_c| map[r][moved_c] != Empty)
                .unwrap();

            if map[r][moved_c] == Hole {
                (r, moved_c)
            } else {
                (r, moved_c - 1)
            }
        }
    }
}
