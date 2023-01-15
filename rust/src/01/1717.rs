use std::fmt::Write;
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
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);

    for (op, a, b) in (0..m).map(|_| (input(), input(), input())) {
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
    // println!("{:?}", disjoint_set.0);
    print!("{output}");
}
