use std::fmt::Write;
use std::io;

const MAX: usize = 1 << 18; // 2^18 = 262_144

struct SegmentTree<const N: usize> {
    tree: [i32; N],
    leaf_start: usize,
}

impl<const N: usize> SegmentTree<N> {
    fn make(n: usize, input: impl Iterator<Item = i32>) -> Self {
        let pow = n.next_power_of_two();
        let mut tree = [0; N];

        for (i, num) in input.enumerate() {
            tree[i + pow] = num;
        }

        for i in (1..pow).rev() {
            tree[i] = tree[i << 1].min(tree[(i << 1) + 1]);
        }

        Self {
            tree,
            leaf_start: pow,
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> i32 {
        left += self.leaf_start;
        right += self.leaf_start;
        let mut result = i32::MAX;

        while left < right {
            if left & 1 == 1 {
                result = result.min(self.tree[left]);
                left += 1;
            }

            if right & 1 == 1 {
                result = result.min(self.tree[right - 1]);
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
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());
    let segment_tree = SegmentTree::<MAX>::make(n, (0..n).map(|_| input()));

    for (a, b) in (0..m).map(|_| (input() as usize - 1, input() as usize)) {
        writeln!(output, "{}", segment_tree.query(a, b)).unwrap();
    }

    print!("{output}");
}
