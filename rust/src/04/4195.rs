use std::collections::HashMap;
use std::fmt::Write;
use std::io;

struct DisjointSet<'a>(HashMap<&'a str, (&'a str, i32)>);

impl<'a> DisjointSet<'a> {
    fn with_capacity(n: usize) -> Self {
        Self(HashMap::with_capacity(n))
    }

    fn insert(&mut self, a: &'a str) {
        self.0.entry(a).or_insert((a, 1));
    }

    fn find(&mut self, a: &'a str) -> (&'a str, i32) {
        let (parent, _) = self.0[a];

        if parent != a {
            let temp = self.find(parent);
            self.0.insert(a, temp);
        }

        self.0[a]
    }

    fn union(&mut self, a: &'a str, b: &'a str) {
        let ((a, a_size), (b, b_size)) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        if a_size > b_size {
            self.0.entry(a).and_modify(|c| (*c).1 += b_size);
            self.0.entry(b).and_modify(|c| (*c).0 = a);
        } else {
            self.0.entry(b).and_modify(|c| (*c).1 += a_size);
            self.0.entry(a).and_modify(|c| (*c).0 = b);
        }
    }

    fn get_size(&mut self, a: &'a str) -> i32 {
        self.find(a).1
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());
        let mut disjoint_set = DisjointSet::with_capacity(n);

        for (friend1, friend2) in (0..n).map(|_| (input(), input())) {
            disjoint_set.insert(friend1);
            disjoint_set.insert(friend2);
            disjoint_set.union(friend1, friend2);

            writeln!(output, "{}", disjoint_set.get_size(friend1)).unwrap();
        }
        // println!("{:?}", disjoint_set.0);
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
