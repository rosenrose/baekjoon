use std::io;

struct DisjointSet<const N: usize>([i32; N]);

impl<const N: usize> DisjointSet<N> {
    fn make() -> Self {
        Self(std::array::from_fn(|i| i as i32))
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

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    let [n, m] = [(); 2].map(|_| input() as usize);
    let mut disjoint_set = DisjointSet::<MAX>::make();

    let mut points = [(0.0, 0.0); MAX];

    for (i, point) in (0..n).map(|_| (input(), input())).enumerate() {
        points[i] = point;
    }

    let mut connected = [[false; MAX]; MAX];

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input() as i32 - 1)) {
        disjoint_set.union(a, b);
        connected[a as usize][b as usize] = true;
    }

    let mut edges = [((0, 0), 0.0); MAX * MAX];
    let mut edges_len = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            if connected[i][j] {
                continue;
            }

            let dist = get_distance(points[i], points[j]);
            edges[edges_len] = ((i as i32, j as i32), dist);
            edges_len += 1;
        }
    }

    edges[..edges_len].sort_unstable_by(|&(_, d1), (_, d2)| d1.total_cmp(d2));

    let min_weight: f64 = edges[..edges_len]
        .iter()
        .filter_map(|&((a, b), dist)| {
            (!disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                dist
            })
        })
        .sum();

    println!("{min_weight:.2}");
}

fn get_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    (x1 - x2).hypot(y1 - y2)
}
