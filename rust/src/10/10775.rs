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
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<usize>);

    let gates = input.next().unwrap();
    let mut disjoint_set = DisjointSet::make(gates);
    let mut count = 0;

    for plane in input.skip(1) {
        let dock = disjoint_set.find(plane);

        if dock == 0 {
            break;
        }

        disjoint_set.union(dock - 1, dock);
        count += 1;
    }
    // println!("{:?}", disjoint_set.0);
    println!("{count}");
}
