use std::io;

struct DisjointSet(Vec<i32>);

impl DisjointSet {
    fn make(n: i32) -> Self {
        Self((0..n).collect())
    }

    fn find(&mut self, a: i32) -> i32 {
        let a_idx = a as usize;

        if self.0[a_idx] != a {
            self.0[a_idx] = self.find(self.0[a_idx]);
        }

        self.0[a_idx]
    }

    fn union(&mut self, a: i32, b: i32) {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.0[b as usize] = a;
    }

    fn is_same(&mut self, a: i32, b: i32) -> bool {
        self.find(a) == self.find(b)
    }
}

const WIDTH_MAX: usize = 1500;
const HEIGHT_MAX: usize = 1500;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<i16>().unwrap());
    let mut map = [[true; WIDTH_MAX]; HEIGHT_MAX];
    let mut disjoint_set = DisjointSet::make(width as i32 * height as i32);

    let mut waters = Vec::new();
    let mut swans = Vec::with_capacity(2);

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            match ch {
                '.' | 'L' => {
                    let idx = get_flat_idx((r as i16, c as i16), width);
                    let adjacents = [(r.saturating_sub(1), c), (r, c.saturating_sub(1))];

                    for (adj_r, adj_c) in adjacents {
                        if map[adj_r][adj_c] {
                            continue;
                        }

                        let adj_idx = get_flat_idx((adj_r as i16, adj_c as i16), width);

                        if disjoint_set.is_same(idx, adj_idx) {
                            continue;
                        }

                        if ch == 'L' {
                            disjoint_set.union(idx, adj_idx);
                        } else {
                            disjoint_set.union(adj_idx, idx);
                        }
                    }

                    waters.push((r as i16, c as i16));

                    if ch == 'L' {
                        swans.push(idx);
                    }

                    map[r][c] = false;
                }
                'X' => (),
                _ => unreachable!(),
            }
        }
    }

    let days = simulate(
        &mut map[..height as usize],
        width,
        disjoint_set,
        waters,
        swans,
    );

    println!("{days}");
}

fn simulate(
    map: &mut [[bool; WIDTH_MAX]],
    width: i16,
    mut disjoint_set: DisjointSet,
    mut waters: Vec<(i16, i16)>,
    swans: Vec<i32>,
) -> i32 {
    let height = map.len() as i16;
    let mut days = 0;
    let mut temp = Vec::new();

    while !disjoint_set.is_same(swans[0], swans[1]) {
        temp.clone_from(&waters);
        waters.clear();

        for &(r, c) in &temp {
            let idx = get_flat_idx((r, c), width);

            for (adj_r, adj_c) in get_adjacents(r, c, width, height) {
                let adj_idx = get_flat_idx((adj_r, adj_c), width);

                if disjoint_set.is_same(idx, adj_idx) {
                    continue;
                }

                if map[adj_r as usize][adj_c as usize] {
                    map[adj_r as usize][adj_c as usize] = false;
                    waters.push((adj_r, adj_c));
                }

                disjoint_set.union(idx, adj_idx);
            }
        }

        // 백조사이 빙판 수가 만약 짝수라면, days 증가 전에 만나는지 확인 필요
        for &(r, c) in &waters {
            let idx = get_flat_idx((r, c), width);

            for (adj_r, adj_c) in get_adjacents(r, c, width, height) {
                let adj_idx = get_flat_idx((adj_r, adj_c), width);

                if map[adj_r as usize][adj_c as usize] || disjoint_set.is_same(idx, adj_idx) {
                    continue;
                }

                disjoint_set.union(idx, adj_idx);
            }
        }

        days += 1;
    }

    days
}

fn get_flat_idx((r, c): (i16, i16), width: i16) -> i32 {
    (r as i32 * width as i32) + c as i32
}

fn get_adjacents(r: i16, c: i16, width: i16, height: i16) -> [(i16, i16); 4] {
    [
        ((r - 1).max(0), c),
        (r, (c - 1).max(0)),
        ((r + 1).min(height - 1), c),
        (r, (c + 1).min(width - 1)),
    ]
}
