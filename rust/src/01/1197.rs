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

const NODES_MAX: usize = 10000 + 1;
const EDGES_MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_v, e] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut disjoint_set = DisjointSet::<NODES_MAX>::make();
    let mut edges = [[0; 3]; EDGES_MAX];

    for (i, edge) in (0..e)
        .map(|_| [(); 3].map(|_| input.next().unwrap()))
        .enumerate()
    {
        edges[i] = edge;
    }

    edges[..e].sort_unstable_by_key(|&[.., weight]| weight);

    let min_weight: i32 = edges[..e]
        .iter()
        .filter_map(|&[a, b, c]| {
            (!disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                c
            })
        })
        .sum();

    println!("{min_weight}");
}
