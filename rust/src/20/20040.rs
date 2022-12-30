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
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);

    for i in 1..=m {
        let (a, b) = (input(), input());

        if disjoint_set.is_same(a, b) {
            println!("{i}");
            return;
        }

        disjoint_set.union(a, b);
        // println!("{:?}", disjoint_set.set);
    }

    println!("0");
}
