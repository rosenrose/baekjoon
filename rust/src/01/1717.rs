use std::fmt::Write;
use std::io;

struct DisjointSet<const N: usize>([i32; N]);

impl<const N: usize> DisjointSet<N> {
    fn make() -> Self {
        Self(std::array::from_fn(|i| i as i32))
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

const MAX: usize = 1_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [_n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut disjoint_set = DisjointSet::<MAX>::make();

    for [op, a, b] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
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

    print!("{output}");
}
