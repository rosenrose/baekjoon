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
    let know_truth: Vec<_> = (0..input()).map(|_| input()).collect();

    if know_truth.is_empty() {
        println!("{m}");
        return;
    }

    let know_truth_group = know_truth[0];
    let mut disjoint_set = DisjointSet::make(n);

    for i in 1..know_truth.len() {
        disjoint_set.union(know_truth_group, know_truth[i]);
    }

    let parties: Vec<Vec<_>> = (0..m)
        .map(|_| (0..input()).map(|_| input()).collect())
        .collect();

    for _ in 0..m - 1 {
        for party in parties.iter() {
            if party
                .iter()
                .any(|&p| disjoint_set.is_same(p, know_truth_group))
            {
                for &p in party {
                    disjoint_set.union(know_truth_group, p);
                }
            }
        }
    }

    let count = parties
        .iter()
        .filter(|party| {
            party
                .iter()
                .all(|&p| !disjoint_set.is_same(p, know_truth_group))
        })
        .count();

    println!("{count}");
}
