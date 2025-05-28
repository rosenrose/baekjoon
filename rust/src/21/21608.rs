use std::cmp::Reverse;
use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut room = [[0; MAX]; MAX];
    let mut students = [(0, [0; 4]); MAX * MAX];

    for i in 0..n * n {
        students[i] = (input(), [(); 4].map(|_| input()));
    }

    let sum = simulate(&mut room[..n], &mut students[..n * n]);

    println!("{sum}");
}

fn simulate(room: &mut [[usize; MAX]], students: &mut [(usize, [usize; 4])]) -> i32 {
    let n = room.len();
    let get_adjacents = |r: usize, c: usize| {
        [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ]
    };

    for &(num, prefers) in students.iter() {
        let mut empty_cells = [(0, 0, (0, 0)); MAX * MAX];
        let mut empty_cells_len = 0;

        for (r, row) in room.iter().enumerate() {
            for (c, &cell) in row[..n].iter().enumerate() {
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

                empty_cells[empty_cells_len] = (prefer_count, empty_count, (r, c));
                empty_cells_len += 1;
            }
        }

        let (.., select) = empty_cells[..empty_cells_len]
            .iter()
            .max_by_key(|(prefer, empty, coord)| (prefer, empty, Reverse(coord)))
            .unwrap();

        room[select.0][select.1] = num;
    }
    // println!("{room:?}");
    students.sort();
    let mut satisfaction = 0;

    for (r, row) in room.iter().enumerate() {
        for (c, &num) in row[..n].iter().enumerate() {
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
