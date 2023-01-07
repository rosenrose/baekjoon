use std::collections::HashMap;
use std::io;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Point<'a>(&'a str, &'a str);

struct DisjointSet<'a> {
    set: HashMap<Point<'a>, Point<'a>>,
}

impl<'a> DisjointSet<'a> {
    fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }

    fn insert(&mut self, a: Point<'a>, b: Point<'a>) {
        self.set.entry(a).or_insert(a);
        self.set.entry(b).or_insert(b);

        self.union(a, b);
    }

    fn find(&mut self, a: Point<'a>) -> Option<Point<'a>> {
        if let Some(&result) = self.set.get(&a) {
            if result != a {
                let parent = self.find(result).unwrap();
                self.set.insert(a, parent);
            }
        }

        self.set.get(&a).copied()
    }

    fn union(&mut self, a: Point<'a>, b: Point<'a>) {
        let (a, b) = (self.find(a).unwrap(), self.find(b).unwrap());

        if a == b {
            return;
        }

        self.set.insert(b, a);
    }

    fn is_same(&mut self, a: Point<'a>, b: Point<'a>) -> bool {
        match (self.find(a), self.find(b)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();

    let mut disjoint_set = DisjointSet::new();
    let points: Vec<_> = (0..n)
        .map(|_| Point(input.next().unwrap(), input.next().unwrap()))
        .collect();

    let mut edges = Vec::new();

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let (p1, p2) = (points[i], points[j]);
            let dist = distance_of_points(
                (parse_float(p1.0), parse_float(p1.1)),
                (parse_float(p2.0), parse_float(p2.1)),
            );

            edges.push((points[i], points[j], dist));
        }
    }

    edges.sort_unstable_by(|&(_, _, d1), (_, _, d2)| d1.total_cmp(d2));

    let min_weight: f32 = edges
        .iter()
        .filter_map(|&(p1, p2, dist)| {
            (!disjoint_set.is_same(p1, p2)).then(|| {
                disjoint_set.insert(p1, p2);
                dist
            })
        })
        .sum();

    println!("{min_weight:.2}");
}

fn distance_of_points((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    (x1 - x2).hypot(y1 - y2)
}

fn parse_float(buf: &str) -> f32 {
    buf.parse().unwrap()
}
