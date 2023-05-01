use std::fmt::Write;
use std::io;

struct SegmentTree {
    tree: Vec<i32>,
    leaf_start: usize,
}

impl SegmentTree {
    fn make(n: usize, mut input: impl Iterator<Item = i32>) -> Self {
        let pow = n.next_power_of_two();
        let mut tree = vec![0; pow << 1];

        for i in pow..pow + n {
            tree[i] = input.next().unwrap();
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
    let segment_tree = SegmentTree::make(n, (0..n).map(|_| input()));

    for (a, b) in (0..m).map(|_| (input() as usize - 1, input() as usize)) {
        writeln!(output, "{}", segment_tree.query(a, b)).unwrap();
    }

    print!("{output}");
}
