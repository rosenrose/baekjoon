use std::io;

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input() as usize, input() as usize, input());
    let mut map = vec![vec![Vec::new(); n]; n];

    for (r, c, m, s, d) in (0..m).map(|_| (input(), input(), input(), input(), input())) {
        map[r as usize - 1][c as usize - 1].push((m, s, d as usize));
    }

    let sum = simulate(map, k);

    println!("{sum}");
}

fn simulate(mut map: Vec<Vec<Vec<(i32, i32, usize)>>>, k: i32) -> i32 {
    for _ in 0..k {
        let moved = move_fireballs(&mut map);
        split_fireballs(&mut map, moved);
        // for r in &map {
        //     println!("{r:?}");
        // }
    }

    map.iter().flatten().flatten().map(|(m, _, _)| m).sum()
}

fn move_fireballs(map: &mut Vec<Vec<Vec<(i32, i32, usize)>>>) -> Vec<Vec<Vec<(i32, i32, usize)>>> {
    let n = map.len();
    let mut moved = vec![vec![Vec::new(); n]; n];

    for (r, row) in map.iter_mut().enumerate() {
        for (c, fireballs) in row.iter_mut().enumerate() {
            while let Some((m, s, d)) = fireballs.pop() {
                let dir = DIRS[d];
                let (moved_r, moved_c) = (
                    (r as i32 + dir.0 * s).rem_euclid(n as i32) as usize,
                    (c as i32 + dir.1 * s).rem_euclid(n as i32) as usize,
                );

                moved[moved_r][moved_c].push((m, s, d));
            }
        }
    }

    moved
}

fn split_fireballs(
    map: &mut Vec<Vec<Vec<(i32, i32, usize)>>>,
    moved: Vec<Vec<Vec<(i32, i32, usize)>>>,
) {
    for (r, row) in moved.iter().enumerate() {
        for (c, fireballs) in row.iter().enumerate() {
            if fireballs.len() == 1 {
                map[r][c].push(fireballs[0]);
                continue;
            }

            let (mut mass, mut speed) = (0, 0);
            let (mut is_all_even, mut is_all_odd) = (true, true);
            let len = moved[r][c].len() as i32;

            for (m, s, d) in fireballs {
                mass += m;
                speed += s;

                if d & 1 == 0 {
                    is_all_odd = false;
                } else {
                    is_all_even = false;
                }
            }

            mass /= 5;

            if mass == 0 {
                continue;
            }

            speed /= len;

            for i in 0..4 {
                map[r][c].push((
                    mass,
                    speed,
                    (i * 2) + usize::from(!(is_all_even || is_all_odd)),
                ));
            }
        }
    }
}
