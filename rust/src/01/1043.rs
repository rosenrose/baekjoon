use std::io;

struct DisjointSet<const N: usize>([usize; N]);

impl<const N: usize> DisjointSet<N> {
    fn make() -> Self {
        Self(std::array::from_fn(|i| i))
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

const PEOPLE_MAX: usize = 50 + 1;
const PARTY_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (_n, m) = (input(), input());
    let know_truth_len = input();
    let mut know_truth = [0; PEOPLE_MAX];

    for i in 0..know_truth_len {
        know_truth[i] = input();
    }

    if know_truth_len == 0 {
        println!("{m}");
        return;
    }

    let know_truth_group = know_truth[0];
    let mut disjoint_set = DisjointSet::<PEOPLE_MAX>::make();

    for i in 1..know_truth.len() {
        disjoint_set.union(know_truth_group, know_truth[i]);
    }

    let mut parties = [(); PARTY_MAX].map(|_| Vec::new());

    for r in 0..m {
        parties[r] = (0..input()).map(|_| input()).collect();
    }

    for _ in 0..m - 1 {
        for party in &parties[..m] {
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

    let count = parties[..m]
        .iter()
        .filter(|party| {
            party
                .iter()
                .all(|&p| !disjoint_set.is_same(p, know_truth_group))
        })
        .count();

    println!("{count}");
}
