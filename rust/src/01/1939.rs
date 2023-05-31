use std::cmp::Reverse;
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
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);

    let mut edges: Vec<_> = (0..m).map(|_| (input(), input(), input() as i32)).collect();
    edges.sort_unstable_by_key(|&(_, _, weight)| Reverse(weight));

    let (start, end) = (input(), input());

    for (a, b, c) in edges {
        disjoint_set.union(a, b);

        if disjoint_set.is_same(start, end) {
            println!("{c}");
            return;
        }
    }
}
