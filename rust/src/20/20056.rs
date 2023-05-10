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
    let mut fireballs = Vec::with_capacity(m);

    for (r, c, m, s, d) in (0..m).map(|_| (input(), input(), input(), input(), input())) {
        let coord = (r as usize - 1, c as usize - 1);
        let info = (m, s, d as usize);

        map[coord.0][coord.1].push(info);
        fireballs.push(coord);
    }

    let sum = simulate(map, fireballs, k);

    println!("{sum}");
}

fn simulate(
    mut map: Vec<Vec<Vec<(i32, i32, usize)>>>,
    mut fireballs: Vec<(usize, usize)>,
    k: i32,
) -> i32 {
    for _ in 0..k {
        let merged_coords = move_fireballs(&mut map, &mut fireballs);

        for (r, c) in merged_coords {
            let (mut mass, mut speed) = (0, 0);
            let (mut is_all_even, mut is_all_odd) = (true, true);
            let len = map[r][c].len() as i32;

            while let Some((m, s, d)) = map[r][c].pop() {
                mass += m;
                speed += s;

                if d & 1 == 0 {
                    is_all_odd = false;
                } else {
                    is_all_even = false;
                }
            }

            fireballs.retain(|&fireball_coord| fireball_coord != (r, c));
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
                fireballs.push((r, c));
            }
        }
        // for r in &map {
        //     println!("{r:?}");
        // }
        // println!("{fireballs:?}\n");
    }

    let mut total_mass = 0;

    for (r, c) in fireballs {
        total_mass += map[r][c].pop().unwrap().0;
    }

    total_mass
}

fn move_fireballs(
    map: &mut Vec<Vec<Vec<(i32, i32, usize)>>>,
    fireballs: &mut Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let n = map.len() as i32;
    let mut fireball_infos = Vec::with_capacity(fireballs.len());
    let mut merged = Vec::new();

    for (r, c) in fireballs.iter_mut() {
        let (m, s, d) = map[*r][*c].pop().unwrap();
        fireball_infos.push((m, s, d));

        let dir = DIRS[d];

        (*r, *c) = (
            (*r as i32 + dir.0 * s).rem_euclid(n) as usize,
            (*c as i32 + dir.1 * s).rem_euclid(n) as usize,
        );
    }

    for (&(r, c), info) in fireballs.iter().zip(fireball_infos) {
        if map[r][c].len() == 1 {
            merged.push((r, c));
        }

        map[r][c].push(info);
    }

    merged
}
