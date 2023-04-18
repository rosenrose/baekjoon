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

    'outer: for _ in 0..input() {
        let (n, m, p, q) = (input(), input(), input(), input());
        let mut disjoint_set = DisjointSet::make(n);
        let mut edges: Vec<_> = (0..m).map(|_| (input(), input(), input())).collect();

        edges.sort_unstable_by_key(|&(_, _, weight)| weight);

        for (u, v, _) in edges {
            if disjoint_set.is_same(u, v) {
                continue;
            }

            disjoint_set.union(u, v);

            if (u, v) == (p, q) || (v, u) == (p, q) {
                println!("YES");
                continue 'outer;
            }
        }

        println!("NO");
    }
}
