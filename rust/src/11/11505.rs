use std::fmt::Write;
use std::io;

const M: i64 = 1_000_000_007;

struct SegmentTree {
    tree: Vec<i64>,
    end: usize,
}

impl SegmentTree {
    fn from(n: usize, mut input: impl Iterator<Item = i64>) -> Self {
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
        tree: &mut Vec<i64>,
        input: &mut impl Iterator<Item = i64>,
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

        tree[node] = (tree[left] * tree[right]) % M;
    }

    fn update(&mut self, idx: usize, val: i64) {
        Self::update_recursive(&mut self.tree, 1, 0, self.end, idx, val);
    }

    fn update_recursive(
        tree: &mut Vec<i64>,
        node: usize,
        start: usize,
        end: usize,
        idx: usize,
        val: i64,
    ) {
        if idx < start || end < idx {
            return;
        }
        if start == end {
            tree[node] = val;
            return;
        }

        let (left, right) = (node << 1, (node << 1) + 1);
        let mid = (start + end) >> 1;

        Self::update_recursive(tree, left, start, mid, idx, val);
        Self::update_recursive(tree, right, mid + 1, end, idx, val);

        tree[node] = (tree[left] * tree[right]) % M;
    }

    fn query(&self, lower: usize, upper: usize) -> i64 {
        Self::query_recursive(&self.tree, 1, 0, self.end, lower, upper)
    }

    fn query_recursive(
        tree: &Vec<i64>,
        node: usize,
        start: usize,
        end: usize,
        lower: usize,
        upper: usize,
    ) -> i64 {
        if end < lower || upper < start {
            return 1;
        }
        if lower <= start && end <= upper {
            return tree[node];
        }

        let (left, right) = (node << 1, (node << 1) + 1);
        let mid = (start + end) >> 1;

        (Self::query_recursive(tree, left, start, mid, lower, upper)
            * Self::query_recursive(tree, right, mid + 1, end, lower, upper))
            % M
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, k) = (input() as usize, input(), input());
    let mut segment_tree = SegmentTree::from(n, (0..n).map(|_| input()));

    for (a, b, c) in (0..m + k).map(|_| (input(), input(), input())) {
        match a {
            1 => segment_tree.update(b as usize - 1, c),
            2 => writeln!(
                output,
                "{}",
                segment_tree.query(b as usize - 1, c as usize - 1)
            )
            .unwrap(),
            _ => (),
        }
    }

    print!("{output}");
}
