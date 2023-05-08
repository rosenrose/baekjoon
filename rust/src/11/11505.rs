use std::fmt::Write;
use std::io;

const M: i64 = 1_000_000_007;

struct SegmentTree {
    tree: Vec<i64>,
    leaf_start: usize,
}

impl SegmentTree {
    fn make(n: usize, mut input: impl Iterator<Item = i64>) -> Self {
        let pow = n.next_power_of_two();
        let mut tree = vec![0; pow << 1];

        for i in pow..pow + n {
            tree[i] = input.next().unwrap();
        }

        for i in (1..pow).rev() {
            tree[i] = (tree[i << 1] * tree[(i << 1) + 1]) % M;
        }

        Self {
            tree,
            leaf_start: pow,
        }
    }

    fn update(&mut self, mut i: usize, val: i64) {
        i += self.leaf_start;
        self.tree[i] = val;

        while i > 1 {
            i >>= 1;
            self.tree[i] = (self.tree[i << 1] * self.tree[(i << 1) + 1]) % M;
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> i64 {
        left += self.leaf_start;
        right += self.leaf_start;
        let mut result = 1;

        while left < right {
            if left & 1 == 1 {
                result = (result * self.tree[left]) % M;
                left += 1;
            }

            if right & 1 == 1 {
                result = (result * self.tree[right - 1]) % M;
                right -= 1;
            }

            left >>= 1;
            right >>= 1;
        }

        result
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, k) = (input() as usize, input(), input());
    let mut segment_tree = SegmentTree::make(n, (0..n).map(|_| input()));

    for (a, b, c) in (0..m + k).map(|_| (input(), input(), input())) {
        match a {
            1 => segment_tree.update(b as usize - 1, c),
            2 => writeln!(output, "{}", segment_tree.query(b as usize - 1, c as usize)).unwrap(),
            _ => unreachable!(),
        }
    }

    print!("{output}");
}
