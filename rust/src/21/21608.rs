use std::cmp::Reverse;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let room = vec![vec![0; n]; n];
    let students: Vec<_> = (0..n * n)
        .map(|_| (input(), [(); 4].map(|_| input())))
        .collect();

    let sum = simulate(room, students);

    println!("{sum}");
}

fn simulate(mut room: Vec<Vec<usize>>, mut students: Vec<(usize, [usize; 4])>) -> i32 {
    let n = room.len();
    let get_adjacents = |r: usize, c: usize| {
        [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ]
    };

    for &(num, prefers) in &students {
        let mut empty_cells = Vec::new();

        for (r, row) in room.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    continue;
                }

                let (mut prefer_count, mut empty_count) = (0, 0);

                for (adj_r, adj_c) in get_adjacents(r, c) {
                    let adj_cell = room[adj_r][adj_c];

                    if prefers.contains(&adj_cell) {
                        prefer_count += 1;
                    }
                    if (adj_r, adj_c) != (r, c) && adj_cell == 0 {
                        empty_count += 1;
                    }
                }

                empty_cells.push((prefer_count, empty_count, (r, c)));
            }
        }

        let (.., select) = empty_cells
            .iter()
            .max_by_key(|(prefer, empty, coord)| (prefer, empty, Reverse(coord)))
            .unwrap();

        room[select.0][select.1] = num;
    }
    // println!("{room:?}");
    students.sort();
    let mut satisfaction = 0;

    for (r, row) in room.iter().enumerate() {
        for (c, &num) in row.iter().enumerate() {
            let prefer_count = get_adjacents(r, c)
                .iter()
                .filter(|&&(adj_r, adj_c)| students[num - 1].1.contains(&room[adj_r][adj_c]))
                .count();

            satisfaction += if prefer_count == 0 {
                0
            } else {
                10_i32.pow(prefer_count as u32 - 1)
            };
        }
    }

    satisfaction
}
