use std::io;

struct DisjointSet {
    set: Vec<usize>,
}

impl DisjointSet {
    fn make(n: usize) -> Self {
        Self {
            set: (0..=n).collect(),
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
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, m) = (input(), input());

        let mut disjoint_set = DisjointSet::make(n);
        let mut count = 0;

        for _ in 0..m {
            let (a, b) = (input(), input());

            if !disjoint_set.is_same(a, b) {
                count += 1;
            }

            disjoint_set.union(a, b);
        }

        println!("{count}");
    }
}
