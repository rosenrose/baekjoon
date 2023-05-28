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

    println!("{}", u8::from(simulate(map, red, blue)));
}

fn simulate(mut map: Vec<Vec<Cells>>, red: (usize, usize), blue: (usize, usize)) -> bool {
    let mut queue = VecDeque::from([((red, blue), 0)]);

    while let Some(((red, blue), time)) = queue.pop_front() {
        if time >= 10 {
            return false;
        }

        map[red.0][red.1] = Red;
        map[blue.0][blue.1] = Blue;

        for dir in [Up, Down, Left, Right] {
            let (moved_red, moved_blue) = match dir {
                Up => {
                    if red.0 < blue.0 {
                        move_balls(red, blue, dir, &mut map)
                    } else {
                        move_balls(blue, red, dir, &mut map)
                    }
                }
                Down => {
                    if red.0 < blue.0 {
                        move_balls(blue, red, dir, &mut map)
                    } else {
                        move_balls(red, blue, dir, &mut map)
                    }
                }
                Left => {
                    if red.1 < blue.1 {
                        move_balls(red, blue, dir, &mut map)
                    } else {
                        move_balls(blue, red, dir, &mut map)
                    }
                }
                Right => {
                    if red.1 < blue.1 {
                        move_balls(blue, red, dir, &mut map)
                    } else {
                        move_balls(red, blue, dir, &mut map)
                    }
                }
            };

            if (moved_red, moved_blue) == (red, blue) || map[moved_blue.0][moved_blue.1] == Hole {
                continue;
            }

            if map[moved_red.0][moved_red.1] == Hole {
                return true;
            }

            queue.push_back(((moved_red, moved_blue), time + 1));
        }

        map[red.0][red.1] = Empty;
        map[blue.0][blue.1] = Empty;
    }

    false
}

fn move_balls(
    first: (usize, usize),
    second: (usize, usize),
    dir: Dirs,
    map: &mut Vec<Vec<Cells>>,
) -> ((usize, usize), (usize, usize)) {
    let first_moved = get_moved_coord(first, dir, map);
    let first_ball = map[first.0][first.1];
    let temp = map[first_moved.0][first_moved.1];

    map[first.0][first.1] = Empty;

    if temp != Hole {
        map[first_moved.0][first_moved.1] = first_ball;
    }

    let second_moved = get_moved_coord(second, dir, map);

    map[first.0][first.1] = first_ball;
    map[first_moved.0][first_moved.1] = temp;

    if first_ball == Red {
        (first_moved, second_moved)
    } else {
        (second_moved, first_moved)
    }
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
