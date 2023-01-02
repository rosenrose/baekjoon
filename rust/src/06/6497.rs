use std::io::{stdin, Read};

struct DisjointSet {
    set: Vec<usize>,
}

impl DisjointSet {
    fn make(n: usize) -> Self {
        Self {
            set: (0..n).collect(),
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.set[a] != a {
            self.set[a] = self.find(self.set[a]);
        }

        self.set[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.set[b] = a;
    }

    fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next();

    while let (Some(m), Some(n)) = (input(), input()) {
        if (m, n) == (0, 0) {
            return;
        }

        let mut disjoint_set = DisjointSet::make(m);
        let mut total_weight = 0;
        let mut edges: Vec<_> = (0..n)
            .map(|_| {
                let (x, y, z) = (input().unwrap(), input().unwrap(), input().unwrap() as i32);
                total_weight += z;
                (x, y, z)
            })
            .collect();

        edges.sort_unstable_by_key(|&(_, _, weight)| weight);

        let min_weight: i32 = edges
            .iter()
            .filter_map(|&(a, b, w)| {
                (!disjoint_set.is_same(a, b)).then(|| {
                    disjoint_set.union(a, b);
                    w
                })
            })
            .sum();

        println!("{}", total_weight - min_weight);
    }
}
