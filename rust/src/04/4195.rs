use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

struct DisjointSet<'a> {
    set: HashMap<&'a str, (&'a str, i32)>,
}

impl<'a> DisjointSet<'a> {
    fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }

    fn find(&mut self, a: &'a str) -> (&'a str, i32) {
        let (result, _) = *self.set.get(a).unwrap();

        if result != a {
            let parent = self.find(result);
            self.set.insert(a, parent);
        }

        *self.set.get(a).unwrap()
    }

    fn insert(&mut self, a: &'a str, b: &'a str) {
        self.set.entry(a).or_insert((a, 1));
        self.set.entry(b).or_insert((b, 1));

        self.union(a, b);
    }

    fn union(&mut self, a: &'a str, b: &'a str) {
        let ((a, a_size), (b, b_size)) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.set.insert(b, (a, a_size));
        self.set.entry(a).and_modify(|c| (*c).1 += b_size);
    }

    fn get_size(&mut self, a: &'a str) -> i32 {
        self.find(a).1
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());
        let mut disjoint_set = DisjointSet::new();

        for _ in 0..n {
            let (friend1, friend2) = (input(), input());

            disjoint_set.insert(friend1, friend2);

            writeln!(output, "{}", disjoint_set.get_size(friend1)).unwrap();
        }
        // println!("{:?}", disjoint_set.set);
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
