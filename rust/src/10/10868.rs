use std::fmt::Write;
use std::io;

struct SegmentTree {
    tree: Vec<i32>,
    end: usize,
}

impl SegmentTree {
    fn from(n: usize, mut input: impl Iterator<Item = i32>) -> Self {
        let mut len = 1;

        while len < n {
            len <<= 1;
        }

        len <<= 1;

        let mut tree = vec![0; len];
        Self::make(&mut tree, &mut input, 1, 0, n - 1);

        Self { tree, end: n - 1 }
    }

    fn make(
        tree: &mut Vec<i32>,
        input: &mut impl Iterator<Item = i32>,
        node: usize,
        start: usize,
        end: usize,
    ) {
        if start == end {
            tree[node] = input.next().unwrap();
            return;
        }

        let (left, right) = (node << 1, (node << 1) + 1);
        let mid = (start + end) >> 1;

        Self::make(tree, input, left, start, mid);
        Self::make(tree, input, right, mid + 1, end);

        tree[node] = tree[left].min(tree[right]);
    }

    fn query(&self, lower: usize, upper: usize) -> i32 {
        Self::query_recursive(&self.tree, 1, 0, self.end, lower, upper)
    }

    fn query_recursive(
        tree: &Vec<i32>,
        node: usize,
        start: usize,
        end: usize,
        lower: usize,
        upper: usize,
    ) -> i32 {
        if end < lower || upper < start {
            return i32::MAX;
        }
        if lower <= start && end <= upper {
            return tree[node];
        }

        let (left, right) = (node << 1, (node << 1) + 1);
        let mid = (start + end) >> 1;

        let (l_min, r_min) = (
            Self::query_recursive(tree, left, start, mid, lower, upper),
            Self::query_recursive(tree, right, mid + 1, end, lower, upper),
        );

        l_min.min(r_min)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let segment_tree = SegmentTree::from(n, (0..n).map(|_| input()));

    for (a, b) in (0..m).map(|_| (input() as usize - 1, input() as usize - 1)) {
        writeln!(output, "{}", segment_tree.query(a, b)).unwrap();
    }

    print!("{output}");
}
