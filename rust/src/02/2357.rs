use std::fmt::Write;
use std::io;

struct SegmentTree {
    tree: Vec<(i32, i32)>,
    leaf_start: usize,
}

impl SegmentTree {
    fn make(n: usize, mut input: impl Iterator<Item = i32>) -> Self {
        let pow = n.next_power_of_two();
        let mut tree = vec![(0, 0); pow << 1];

        for i in pow..pow + n {
            let num = input.next().unwrap();
            tree[i] = (num, num);
        }

        for i in (1..pow).rev() {
            let (l_min, l_max) = tree[i << 1];
            let (r_min, r_max) = tree[(i << 1) + 1];

            tree[i] = (l_min.min(r_min), l_max.max(r_max));
        }

        Self {
            tree,
            leaf_start: pow,
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> (i32, i32) {
        left += self.leaf_start;
        right += self.leaf_start;
        let mut result = (i32::MAX, i32::MIN);

        while left < right {
            if left & 1 == 1 {
                let (l_min, l_max) = self.tree[left];
                result = (result.0.min(l_min), result.1.max(l_max));
                left += 1;
            }

            if right & 1 == 1 {
                let (r_min, r_max) = self.tree[right - 1];
                result = (result.0.min(r_min), result.1.max(r_max));
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
        let (min, max) = segment_tree.query(a, b);

        writeln!(output, "{min} {max}",).unwrap();
    }

    print!("{output}");
}
