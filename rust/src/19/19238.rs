use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
enum Cells {
    Empty,
    Wall,
    Passenger((usize, usize)),
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, m, fuel] = [(); 3].map(|_| input());
    let mut map: Vec<Vec<_>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| match input() {
                    0 => Cells::Empty,
                    1 => Cells::Wall,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let taxi = (input() - 1, input() - 1);

    for [start_r, start_c, end_r, end_c] in (0..m).map(|_| [(); 4].map(|_| input() - 1)) {
        map[start_r][start_c] = Cells::Passenger((end_r, end_c));
    }

    let fuel_remain = simulate(map, fuel as u32, taxi, m);

    println!("{}", fuel_remain.unwrap_or(-1));
}

fn simulate(
    mut map: Vec<Vec<Cells>>,
    mut fuel: u32,
    mut taxi: (usize, usize),
    mut passenger_count: usize,
) -> Option<i32> {
    let n = map.len();

    loop {
        let passengers = get_passengers(&map, taxi);
        let &(dist_to_start, start) = passengers.iter().min()?;

        let Cells::Passenger(end) = map[start.0][start.1] else {
            unreachable!()
        };

        fuel = fuel.checked_sub(dist_to_start)?;
        taxi = start;
        map[start.0][start.1] = Cells::Empty;

        let dist_to_end = 'a: {
            let mut visited = vec![vec![false; n]; n];
            visited[taxi.0][taxi.1] = true;

            let mut queue = VecDeque::from([(taxi, 0)]);

            while let Some(((r, c), dist)) = queue.pop_front() {
                let new_dist = dist + 1;

                for (adj_r, adj_c) in get_adjacents(r, c, n) {
                    if (adj_r, adj_c) == end {
                        break 'a new_dist;
                    }

                    if is_pass(adj_r, adj_c, &visited, &map) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    queue.push_back(((adj_r, adj_c), new_dist));
                }
            }

            return None;
        };

        fuel = fuel.checked_sub(dist_to_end)? + (dist_to_end * 2);
        taxi = end;
        passenger_count -= 1;

        if passenger_count == 0 {
            return Some(fuel as i32);
        }
    }
}

fn get_passengers(map: &[Vec<Cells>], taxi: (usize, usize)) -> Vec<(u32, (usize, usize))> {
    let n = map.len();
    let mut passengers = Vec::new();
    let mut visited = vec![vec![false; n]; n];
    visited[taxi.0][taxi.1] = true;

    let mut queue = VecDeque::from([(taxi, 0)]);

    while let Some(((r, c), dist)) = queue.pop_front() {
        if let Cells::Passenger(_) = map[r][c] {
            passengers.push((dist, (r, c)));
        }

        for (adj_r, adj_c) in get_adjacents(r, c, n) {
            if is_pass(adj_r, adj_c, &visited, map) {
                continue;
            }

            visited[adj_r][adj_c] = true;
            queue.push_back(((adj_r, adj_c), dist + 1));
        }
    }

    passengers
}

fn get_adjacents(r: usize, c: usize, n: usize) -> [(usize, usize); 4] {
    [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(n - 1), c),
        (r, (c + 1).min(n - 1)),
    ]
}

fn is_pass(r: usize, c: usize, visited: &[Vec<bool>], map: &[Vec<Cells>]) -> bool {
    visited[r][c] || matches!(map[r][c], Cells::Wall)
}
