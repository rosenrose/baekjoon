use std::fmt::Write;
use std::io;

const MAX: usize = 1 << 21; // 2^21 = 2_097_152

struct SegmentTree<const N: usize> {
    tree: [i64; N],
    leaf_start: usize,
}

impl<const N: usize> SegmentTree<N> {
    fn make(n: usize, input: impl Iterator<Item = i64>) -> Self {
        let pow = n.next_power_of_two();
        let mut tree = [0; N];

        for (i, num) in input.take(n).enumerate() {
            tree[i + pow] = num;
        }

        for i in (1..pow).rev() {
            tree[i] = tree[i << 1] + tree[(i << 1) + 1];
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
            self.tree[i] = self.tree[i << 1] + self.tree[(i << 1) + 1];
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> i64 {
        left += self.leaf_start;
        right += self.leaf_start;
        let mut result = 0;

        while left < right {
            if left & 1 == 1 {
                result += self.tree[left];
                left += 1;
            }

            if right & 1 == 1 {
                result += self.tree[right - 1];
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
    let mut output = String::new();

    let [n, m, k] = [(); 3].map(|_| input.next().unwrap());
    let mut segment_tree = SegmentTree::<MAX>::make(n as usize, input.by_ref());

    for [a, b, c] in (0..m + k).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        match a {
            1 => segment_tree.update(b as usize - 1, c),
            2 => writeln!(output, "{}", segment_tree.query(b as usize - 1, c as usize)).unwrap(),
            _ => unreachable!(),
        }
        // println!("{:?}", segment_tree.tree);
    }

    print!("{output}");
}
