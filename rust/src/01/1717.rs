use std::fmt::Write;
use std::io;

struct DisjointSet(Vec<i32>);

impl DisjointSet {
    fn make(n: i32) -> Self {
        Self((0..=n).collect())
    }

    fn find(&mut self, a: i32) -> i32 {
        let a_idx = a as usize;

        if self.0[a_idx] != a {
            self.0[a_idx] = self.find(self.0[a_idx]);
        }

        self.0[a_idx]
    }

    fn union(&mut self, a: i32, b: i32) {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.0[b as usize] = a;
    }

    fn is_same(&mut self, a: i32, b: i32) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);

    for [op, a, b] in (0..m).map(|_| [(); 3].map(|_| input())) {
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
            _ => unreachable!(),
        }
    }
    // println!("{:?}", disjoint_set.0);
    print!("{output}");
}
