use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

struct Heap<T>(Vec<T>);

impl<T> Heap<T>
where
    T: Copy + std::cmp::PartialOrd + std::fmt::Debug,
{
    fn new() -> Self {
        Self(Vec::new())
    }

    fn get_parent(&self, i: usize) -> Option<(usize, T)> {
        if i > 0 {
            let idx = (i - 1) / 2;
            Some((idx, self.0[idx]))
        } else {
            None
        }
    }

    fn get_children(&self, i: usize) -> (Option<(usize, T)>, Option<(usize, T)>) {
        let left_idx = i * 2 + 1;
        let right_idx = i * 2 + 2;

        let left = match self.0.get(left_idx) {
            Some(v) => Some((left_idx, *v)),
            None => None,
        };
        let right = match self.0.get(right_idx) {
            Some(v) => Some((right_idx, *v)),
            None => None,
        };

        (left, right)
    }

    fn push(&mut self, value: T) {
        self.0.push(value);

        Self::up_heapify(self, self.0.len() - 1, value);
    }

    fn pop(&mut self) -> Option<T> {
        if self.0.is_empty() {
            return None;
        }

        let len = self.0.len();

        if len > 1 {
            self.0.swap(0, len - 1);
        }

        let ret = self.0.pop().unwrap();

        Self::down_heapify(self, 0);

        Some(ret)
    }

    fn up_heapify(&mut self, i: usize, value: T) {
        match Self::get_parent(self, i) {
            Some((parent_idx, parent_value)) => {
                if parent_value < value {
                    self.0.swap(parent_idx, i);
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

        if child_val > self.0[i] {
            self.0.swap(i, child_idx);
            Self::down_heapify(self, child_idx);
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut arr: Vec<_> = input.collect();

    merge_sort(&mut arr[..], n);

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

    // let mut heap = Heap::new();
    // lines.for_each(|line| heap.push(line.parse::<i32>().unwrap()));

    // // let mut heap: BinaryHeap<i32> = lines.map(|line| line.parse().unwrap()).collect();

    // let mut arr = Vec::new();

    // while !heap.is_empty() {
    //     arr.push(heap.pop().unwrap());
    // }

    // for num in arr.iter().rev() {
    //     writeln!(output, "{num}").unwrap();
    // }

    print!("{output}");
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
