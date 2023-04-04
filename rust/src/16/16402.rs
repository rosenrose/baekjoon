use std::collections::HashMap;
use std::fmt::Write;
use std::io;

struct DisjointSet<'a>(HashMap<&'a str, &'a str>);

impl<'a> DisjointSet<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, a: &'a str, b: &'a str) {
        self.0.entry(a).or_insert(a);
        self.0.entry(b).or_insert(b);

        self.union(a, b);
    }

    fn find(&mut self, a: &'a str) -> &'a str {
        let result = (*self).0[a];

        if result != a {
            let parent = self.find(result);
            self.0.insert(a, parent);
        }

        (*self).0[a]
    }

    fn union(&mut self, a: &'a str, b: &'a str) {
        let (a_parent, b_parent) = (self.find(a), self.find(b));

        if a_parent == b_parent {
            self.0.insert(a, a);
            self.0.insert(b, a);
        } else {
            self.0.insert(b_parent, a_parent);
        }
    }
}

const KINGDOM_OF: &str = "Kingdom of";

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let (n, _) = input.next().unwrap().split_once(' ').unwrap();
    let n: usize = n.parse().unwrap();
    let mut disjoint_set = DisjointSet::new();

    for kingdom in input.by_ref().take(n) {
        let name = get_name(kingdom);
        disjoint_set.insert(name, name);
    }

    for war in input {
        let mut tokens = war.split(',');
        let (name1, name2, winner) = (
            get_name(tokens.next().unwrap()),
            get_name(tokens.next().unwrap()),
            tokens.next().unwrap(),
        );

        match winner {
            "1" => disjoint_set.union(name1, name2),
            "2" => disjoint_set.union(name2, name1),
            _ => (),
        }
        // println!("{:?}", disjoint_set.0);
    }

    let mut masters: Vec<_> = disjoint_set
        .0
        .iter()
        .filter_map(|(slave, master)| (slave == master).then_some(master))
        .collect();
    masters.sort();

    writeln!(output, "{}", masters.len()).unwrap();

    for master in masters {
        writeln!(output, "{KINGDOM_OF} {master}").unwrap();
    }

    print!("{output}");
}

fn get_name<'a>(kingdom: &'a str) -> &'a str {
    &kingdom[KINGDOM_OF.len() + 1..]
}
