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
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (n, m) = (parse_int(input()), parse_int(input()));
    let schools: Vec<_> = (0..n).map(|_| input()).collect();

    let mut disjoint_set = DisjointSet::make(n);
    let mut edges: Vec<_> = (0..m)
        .map(|_| (parse_int(input()), parse_int(input()), parse_int(input())))
        .collect();

    edges.sort_unstable_by_key(|&(_, _, weight)| weight);

    let min_spanning_tree: Vec<_> = edges
        .iter()
        .filter_map(|&(a, b, dist)| {
            (schools[a - 1] != schools[b - 1] && !disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                dist
            })
        })
        .collect();

    if min_spanning_tree.len() < n - 1 {
        println!("-1");
        return;
    }

    println!("{}", min_spanning_tree.iter().sum::<usize>());
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
