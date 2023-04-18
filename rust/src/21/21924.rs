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
    let mut total_weight = 0;
    let mut edges: Vec<_> = (0..m)
        .map(|_| {
            let (a, b, c) = (input(), input(), input());
            total_weight += c;

            (a, b, c)
        })
        .collect();

    edges.sort_unstable_by_key(|&(_, _, weight)| weight);

    let mut count = 0;
    let min_weight: usize = edges
        .iter()
        .filter_map(|&(a, b, w)| {
            (!disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                count += 1;

                w
            })
        })
        .sum();

    if count < n - 1 {
        println!("-1");
        return;
    }

    println!("{}", total_weight - min_weight);
}
