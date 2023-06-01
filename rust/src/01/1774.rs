use std::collections::HashSet;
use std::io;

struct DisjointSet(Vec<usize>);

impl DisjointSet {
    fn make(n: usize) -> Self {
        Self((0..=n).collect())
    }

    fn find(&mut self, a: usize) -> usize {
        if self.0[a] != a {
            self.0[a] = self.find(self.0[a]);
        }

        self.0[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.0[b] = a;
    }

    fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input() as usize, input() as usize);
    let mut disjoint_set = DisjointSet::make(n);

    let points: Vec<_> = (0..n).map(|_| (input(), input())).collect();
    let connected: HashSet<_> = (0..m)
        .map(|_| {
            let (a, b) = (input() as usize - 1, input() as usize - 1);
            disjoint_set.union(a, b);

            (a, b)
        })
        .collect();

    let mut edges = Vec::new();

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            if connected.contains(&(i, j)) {
                continue;
            }

            let dist = get_distance(points[i], points[j]);
            edges.push(((i, j), dist));
        }
    }

    edges.sort_unstable_by(|&(_, d1), (_, d2)| d1.total_cmp(d2));

    let min_weight: f64 = edges
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
