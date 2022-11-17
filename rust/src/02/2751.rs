use std::io::{stdin, stdout, BufWriter, Read, Write};

struct Heap<T> {
    tree: Vec<T>,
}

impl<T> Heap<T>
where
    T: Copy + std::cmp::PartialOrd + std::fmt::Debug,
{
    fn new() -> Self {
        Self { tree: Vec::new() }
    }

    fn get_parent(&self, i: usize) -> Option<(usize, T)> {
        if i > 0 {
            let idx = (i - 1) / 2;
            Some((idx, self.tree[idx]))
        } else {
            None
        }
    }

    fn get_children(&self, i: usize) -> (Option<(usize, T)>, Option<(usize, T)>) {
        let left_idx = i * 2 + 1;
        let right_idx = i * 2 + 2;

        let left = match self.tree.get(left_idx) {
            Some(v) => Some((left_idx, *v)),
            None => None,
        };
        let right = match self.tree.get(right_idx) {
            Some(v) => Some((right_idx, *v)),
            None => None,
        };

        (left, right)
    }

    fn insert(&mut self, value: T) {
        self.tree.push(value);

        Self::up_heapify(self, self.tree.len() - 1, value);
    }

    fn remove(&mut self) -> Option<T> {
        if self.tree.is_empty() {
            return None;
        }

        let len = self.tree.len();

        if len > 1 {
            self.tree.swap(0, len - 1);
        }

        let ret = self.tree.pop().unwrap();

        Self::down_heapify(self, 0);

        Some(ret)
    }

    fn up_heapify(&mut self, i: usize, value: T) {
        match Self::get_parent(self, i) {
            Some((parent_idx, parent_value)) => {
                if parent_value < value {
                    self.tree.swap(parent_idx, i);
                }

                Self::up_heapify(self, parent_idx, value);
            }
            None => return,
        }
    }

    fn down_heapify(&mut self, i: usize) {
        let (child_idx, child_val) = match Self::get_children(self, i) {
            (Some((left_idx, left_val)), Some((right_idx, right_val))) => {
                if left_val > right_val {
                    (left_idx, left_val)
                } else {
                    (right_idx, right_val)
                }
            }
            (Some((left_idx, left_val)), None) => (left_idx, left_val),
            (None, Some((right_idx, right_val))) => (right_idx, right_val),
            (None, None) => return,
        };

        if child_val > self.tree[i] {
            self.tree.swap(i, child_idx);
            Self::down_heapify(self, child_idx);
        }
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    // let mut arr: Vec<i32> = lines.map(|line| line.parse().unwrap()).collect();

    // merge_sort(&mut arr[..], n);

    // for num in arr {
    //     writeln!(stdout, "{num}").unwrap();
    // }

    let mut arr = Heap::new();
    lines.for_each(|line| arr.insert(line.parse::<i32>().unwrap()));

    let mut sorted = Vec::new();

    while !arr.is_empty() {
        sorted.push(arr.remove().unwrap());
    }

    for num in sorted.iter().rev() {
        writeln!(stdout, "{num}").unwrap();
    }
}

fn merge_sort(arr: &mut [i32], len: usize) {
    if len <= 1 {
        return;
    }

    let pivot = (len / 2) as usize;

    merge_sort(&mut arr[..pivot], pivot);
    merge_sort(&mut arr[pivot..], len - pivot);

    let mut temp = vec![0; len];
    let mut a = 0;
    let mut b = pivot;

    for i in 0..len {
        if a < pivot && b < len {
            if arr[a] < arr[b] {
                temp[i] = arr[a];
                a += 1;
            } else {
                temp[i] = arr[b];
                b += 1;
            }
        } else {
            if a == pivot {
                temp[i] = arr[b];
                b += 1;
            } else {
                temp[i] = arr[a];
                a += 1;
            }
        }
    }

    arr.copy_from_slice(&temp[..]);
}
