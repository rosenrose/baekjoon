use std::cmp::Reverse;
use std::collections::HashSet;
use std::io;

const DIRS: [(i8, i8); 8] = [
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];
const SIZE: usize = 4;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (m, s) = (input(), input());
    let mut map = [(); SIZE].map(|_| [(); SIZE].map(|_| (Vec::new(), 0)));

    for [fx, fy, d] in (0..m).map(|_| [(); 3].map(|_| input() - 1)) {
        map[fx][fy].0.push(d as u8);
    }

    let shark = (input() - 1, input() - 1);
    let count = simulate(map, s, shark);

    println!("{count}");
}

fn simulate(mut map: [[(Vec<u8>, i8); SIZE]; SIZE], s: usize, mut shark: (usize, usize)) -> usize {
    for _ in 0..s {
        let mut copied = [(); SIZE].map(|_| [(); SIZE].map(|_| Vec::new()));

        for r in 0..SIZE {
            for c in 0..SIZE {
                for &dir in &map[r][c].0 {
                    copied[r][c].push(dir);
                }
            }
        }

        move_fishes(&mut map, shark);

        for r in 0..SIZE {
            for c in 0..SIZE {
                if map[r][c].1 > 0 {
                    map[r][c].1 -= 1;
                }
            }
        }

        move_shark(&mut map, &mut shark);

        for r in 0..SIZE {
            for c in 0..SIZE {
                while let Some(dir) = copied[r][c].pop() {
                    map[r][c].0.push(dir)
                }
            }
        }
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("{shark:?}\n");
    }

    map.iter().flatten().map(|(fishes, _)| fishes.len()).sum()
}

fn move_fishes(map: &mut [[(Vec<u8>, i8); SIZE]], shark: (usize, usize)) {
    let mut moved = [(); SIZE].map(|_| [(); SIZE].map(|_| Vec::new()));

    for r in 0..SIZE {
        for c in 0..SIZE {
            while let Some(mut dir) = map[r][c].0.pop() {
                let (mut moved_r, mut moved_c) = (r, c);

                for _ in 0..DIRS.len() {
                    if let Some(coord) = get_moved_coord((r, c), dir) {
                        if coord != shark && map[coord.0][coord.1].1 == 0 {
                            (moved_r, moved_c) = coord;
                            break;
                        }
                    }

                    dir = (dir + (DIRS.len() as u8 - 1)) % (DIRS.len() as u8);
                }

                moved[moved_r][moved_c].push(dir);
            }
        }
    }

    for r in 0..SIZE {
        for c in 0..SIZE {
            while let Some(dir) = moved[r][c].pop() {
                map[r][c].0.push(dir)
            }
        }
    }
}

fn get_moved_coord((r, c): (usize, usize), dir: u8) -> Option<(usize, usize)> {
    let dir = dir as usize;
    let moved = (r as i8 + DIRS[dir].0, c as i8 + DIRS[dir].1);

    matches!(moved, (0..=3, 0..=3)).then_some((moved.0 as usize, moved.1 as usize))
}

fn move_shark(map: &mut [[(Vec<u8>, i8); SIZE]], shark: &mut (usize, usize)) {
    let up_left_down_right = [2, 0, 6, 4];
    let mut moves = Vec::with_capacity(4 * 4 * 4);

    for a in 0..4 {
        for b in 0..4 {
            'outer: for c in 0..4 {
                let indices = [a, b, c];
                let mut coord = *shark;
                let mut path = [(0, 0); 3];

                for (i, &idx) in indices.iter().enumerate() {
                    let Some(moved) = get_moved_coord(coord, up_left_down_right[idx]) else {
                        continue 'outer;
                    };

                    coord = moved;
                    path[i] = moved;
                }

                let count: usize = HashSet::<(usize, usize)>::from_iter(path)
                    .iter()
                    .map(|&(r, c)| map[r][c].0.len())
                    .sum();

                moves.push((count, indices, path));
            }
        }
    }

    let (.., path) = moves
        .iter()
        .max_by_key(|(count, indices, _)| (count, Reverse(indices)))
        .unwrap();

    for &(r, c) in path {
        *shark = (r, c);

        if map[r][c].0.is_empty() {
            continue;
        }

        map[r][c].0.clear();
        map[r][c].1 = 2;
    }
}
