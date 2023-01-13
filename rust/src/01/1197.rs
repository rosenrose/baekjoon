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
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (v, e) = (input() as usize, input());

    let mut disjoint_set = DisjointSet::make(v);
    let mut edges: Vec<_> = (0..e)
        .map(|_| (input() as usize, input() as usize, input()))
        .collect();

    edges.sort_unstable_by_key(|&(_, _, weight)| weight);

    let min_weight: i32 = edges
        .iter()
        .filter_map(|&(a, b, c)| {
            (!disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                c
            })
        })
        .sum();

    println!("{min_weight}");
}
