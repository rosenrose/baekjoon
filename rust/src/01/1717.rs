use std::fmt::Write;
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
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);

    for _ in 0..m {
        let (op, a, b) = (input(), input(), input());

        match op {
            0 => disjoint_set.union(a, b),
            1 => writeln!(
                output,
                "{}",
                if disjoint_set.is_same(a, b) {
                    "YES"
                } else {
                    "NO"
                }
            )
            .unwrap(),
            _ => (),
        }
    }
    // println!("{:?}", disjoint_set.set);
    print!("{output}");
}
