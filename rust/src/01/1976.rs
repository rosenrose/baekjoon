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

const CITY_MAX: usize = 200 + 1;
const PATH_MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, m] = [(); 2].map(|_| input());
    let mut disjoint_set = DisjointSet::<CITY_MAX>::make();

    for i in 1..=n {
        for j in 1..=n {
            if input() == 1 {
                disjoint_set.union(i, j);
            }
        }
    }

    let mut path = [0; PATH_MAX];

    for p in &mut path[..m] {
        *p = input();
    }

    let is_connected = (1..m).all(|i| disjoint_set.is_same(path[i - 1], path[i]));

    println!("{}", if is_connected { "YES" } else { "NO" })
}
