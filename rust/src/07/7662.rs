use std::fmt::Write;
use std::io;

struct MinMaxHeap<T>(Vec<T>);

impl<T: Ord + Copy> MinMaxHeap<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn is_min_level(&self, n: usize) -> bool {
        (n + 1).ilog2() & 1 == 0
        // n += 1;
        // let mut ilog2 = 0;

        // while n > 1 {
        //     n >>= 1;
        //     ilog2 += 1;
        // }

        // ilog2 & 1 == 0
    }

    fn get_parent_idx(&self, i: usize) -> Option<usize> {
        (i > 0).then(|| (i - 1) >> 1)
    }
    fn get_grandparent_idx(&self, i: usize) -> Option<usize> {
        (i > 2).then(|| (i - 3) >> 2)
    }

    fn get_child_or_grandchild_idx_min(&self, i: usize) -> Option<usize> {
        let double = (i + 1) * 2;
        let quad = (i + 1) * 4;
        let len = self.len();

        [double, double + 1, quad, quad + 1, quad + 2, quad + 3]
            .iter()
            .filter_map(|idx| (idx - 1 < len).then_some(idx - 1))
            .min_by_key(|&idx| self.0[idx])
    }
    fn get_child_or_grandchild_idx_max(&self, i: usize) -> Option<usize> {
        let double = (i + 1) * 2;
        let quad = (i + 1) * 4;
        let len = self.len();

        [double, double + 1, quad, quad + 1, quad + 2, quad + 3]
            .iter()
            .filter_map(|idx| (idx - 1 < len).then_some(idx - 1))
            .max_by_key(|&idx| self.0[idx])
    }

    fn is_direct_child(&self, child: usize, parent: usize) -> bool {
        let double = (parent + 1) * 2;
        double - 1 <= child && child <= double
    }

    fn get_max_idx(&self) -> Option<usize> {
        (!self.is_empty()).then(|| {
            let (left, right) = (self.0.get(1), self.0.get(2));

            match (left, right) {
                (None, _) => 0,
                (_, None) => 1,
                (Some(a), Some(b)) => {
                    if a > b {
                        1
                    } else {
                        2
                    }
                }
            }
        })
    }

    fn push(&mut self, value: T) {
        self.0.push(value);
        self.up_heapify(self.len() - 1);
    }

    fn pop_min(&mut self) -> Option<T> {
        self.pop(0)
    }
    fn pop_max(&mut self) -> Option<T> {
        self.pop(self.get_max_idx()?)
    }
    fn pop(&mut self, i: usize) -> Option<T> {
        let len = self.len();

        if len > i + 1 {
            self.0.swap(i, len - 1);
        }

        let value = self.0.pop();
        self.down_heapify(i);

        value
    }

    fn peek_min(&self) -> Option<&T> {
        self.0.get(0)
    }
    fn peek_max(&self) -> Option<&T> {
        self.0.get(self.get_max_idx()?)
    }

    fn up_heapify(&mut self, i: usize) {
        let Some(parent) = self.get_parent_idx(i) else {
            return;
        };

        if self.is_min_level(i) {
            if self.0[parent] < self.0[i] {
                self.0.swap(parent, i);
                self.up_heapify_max(parent);
            } else {
                self.up_heapify_min(i);
            }
        } else {
            if self.0[parent] > self.0[i] {
                self.0.swap(parent, i);
                self.up_heapify_min(parent);
            } else {
                self.up_heapify_max(i);
            }
        }
    }
    fn up_heapify_min(&mut self, i: usize) {
        let Some(grandparent) = self.get_grandparent_idx(i) else {
            return;
        };

        if self.0[grandparent] > self.0[i] {
            self.0.swap(grandparent, i);
            self.up_heapify_min(grandparent);
        }
    }
    fn up_heapify_max(&mut self, i: usize) {
        let Some(grandparent) = self.get_grandparent_idx(i) else {
            return;
        };

        if self.0[grandparent] < self.0[i] {
            self.0.swap(grandparent, i);
            self.up_heapify_max(grandparent);
        }
    }

    fn down_heapify(&mut self, i: usize) {
        if self.is_min_level(i) {
            self.down_heapify_min(i);
        } else {
            self.down_heapify_max(i);
        }
    }
    fn down_heapify_min(&mut self, i: usize) {
        let Some(child) = self.get_child_or_grandchild_idx_min(i) else {
            return;
        };

        if self.0[child] < self.0[i] {
            self.0.swap(child, i);

            if self.is_direct_child(child, i) {
                return;
            }

            let parent = self.get_parent_idx(child).unwrap();

            if self.0[parent] < self.0[child] {
                self.0.swap(parent, child);
            }

            self.down_heapify_min(child);
        }
    }
    fn down_heapify_max(&mut self, i: usize) {
        let Some(child) = self.get_child_or_grandchild_idx_max(i) else {
            return;
        };

        if self.0[child] > self.0[i] {
            self.0.swap(child, i);

            if self.is_direct_child(child, i) {
                return;
            }

            let parent = self.get_parent_idx(child).unwrap();

            if self.0[parent] > self.0[child] {
                self.0.swap(parent, child);
            }

            self.down_heapify_max(child);
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut heap = MinMaxHeap::new();

    for _ in 0..parse_int(input()) {
        for (op, arg) in (0..parse_int(input())).map(|_| (input(), input())) {
            match op {
                "I" => heap.push(parse_int(arg)),
                "D" => match arg {
                    "-1" => {
                        heap.pop_min();
                    }
                    "1" => {
                        heap.pop_max();
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        if let (Some(min), Some(max)) = (heap.peek_min(), heap.peek_max()) {
            writeln!(output, "{max} {min}")
        } else {
            writeln!(output, "EMPTY")
        }
        .unwrap();

        heap.clear();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
