use std::io;

const SIZE: usize = 4;
const DIRS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];
const SHARK: usize = 0;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut map = [[None; SIZE]; SIZE];
    let mut fishes = [None; (SIZE * SIZE) + 1];

    for r in 0..SIZE {
        for c in 0..SIZE {
            let (num, dir_idx) = (input.next().unwrap(), input.next().unwrap() - 1);

            map[r][c] = Some((num, dir_idx));
            fishes[num] = Some((dir_idx, (r, c)));
        }
    }

    let (num, dir_idx) = map[0][0].unwrap();

    map[0][0] = Some((SHARK, dir_idx));
    fishes[SHARK] = Some((dir_idx, (0, 0)));
    fishes[num] = None;
    // println!("{map:?}\n{fishes:?}");
    let max_sum = simulate(map, fishes, num);

    println!("{max_sum}");
}

fn simulate(
    mut map: [[Option<(usize, usize)>; 4]; 4],
    mut fishes: [Option<(usize, (usize, usize))>; 17],
    sum: usize,
) -> usize {
    move_fishes(&mut map, &mut fishes);

    let (shark_dir_idx, (shark_r, shark_c)) = fishes[SHARK].unwrap();
    let shark_path = (1..)
        .map_while(|len| get_moved_coord((shark_r, shark_c), shark_dir_idx, len))
        .filter(|&(r, c)| map[r][c].is_some());

    shark_path
        .map(|(next_r, next_c)| {
            let (fish_num, fish_dir_idx) = map[next_r][next_c].unwrap();
            let (mut map, mut fishes) = (map.clone(), fishes.clone());

            map[next_r][next_c] = Some((SHARK, fish_dir_idx));
            map[shark_r][shark_c] = None;
            fishes[SHARK] = Some((fish_dir_idx, (next_r, next_c)));
            fishes[fish_num] = None;

            simulate(map, fishes, sum + fish_num)
        })
        .max()
        .unwrap_or(sum)
}

fn move_fishes(
    map: &mut [[Option<(usize, usize)>; 4]; 4],
    fishes: &mut [Option<(usize, (usize, usize))>; 17],
) {
    for num in 1..fishes.len() {
        let Some((mut dir_idx, (r, c))) = fishes[num] else {
            continue;
        };

        let (moved_r, moved_c);

        loop {
            if let Some(moved) = get_moved_coord((r, c), dir_idx, 1) {
                if !matches!(map[moved.0][moved.1], Some((SHARK, _))) {
                    (moved_r, moved_c) = moved;
                    break;
                }
            }

            dir_idx = (dir_idx + 1) % DIRS.len();
        }

        fishes[num] = Some((dir_idx, (moved_r, moved_c)));
        map[r][c] = Some((num, dir_idx));

        if let Some((other_num, other_dir_idx)) = map[moved_r][moved_c] {
            fishes[other_num] = Some((other_dir_idx, (r, c)));
        }
        (map[r][c], map[moved_r][moved_c]) = (map[moved_r][moved_c], map[r][c]);
        // for r in map.iter() {
        //     println!("{r:?}");
        // }
        // println!("");
    }
}

fn get_moved_coord((r, c): (usize, usize), dir_idx: usize, len: i8) -> Option<(usize, usize)> {
    let dir = DIRS[dir_idx];
    let moved = (r as i8 + (dir.0 * len), c as i8 + (dir.1 * len));

    matches!(moved, (0..=3, 0..=3)).then_some((moved.0 as usize, moved.1 as usize))
}
