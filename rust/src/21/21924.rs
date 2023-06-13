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

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::make(n);
    let mut total_weight = 0;
    let mut edges: Vec<_> = (0..m)
        .map(|_| {
            let [a, b, c] = [(); 3].map(|_| input());
            total_weight += c as i64;

            [a, b, c]
        })
        .collect();

    edges.sort_unstable_by_key(|&[.., weight]| weight);

    let mut count = 0;
    let min_weight: i64 = edges
        .iter()
        .filter_map(|&[a, b, w]| {
            (!disjoint_set.is_same(a, b)).then(|| {
                disjoint_set.union(a, b);
                count += 1;

                w as i64
            })
        })
        .sum();

    if count < n - 1 {
        println!("-1");
        return;
    }

    println!("{}", total_weight - min_weight);
}
