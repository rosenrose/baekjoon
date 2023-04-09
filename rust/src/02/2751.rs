use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

struct Heap<T: Ord>(Vec<T>);

impl<T: Ord> Heap<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_parent_idx(&self, i: usize) -> Option<usize> {
        (i > 0).then_some((i - 1) / 2)
    }

    fn get_children_idx(&self, i: usize) -> (Option<usize>, Option<usize>) {
        let left_idx = i * 2 + 1;
        let right_idx = i * 2 + 2;
        let len = self.len();

        (
            (left_idx < len).then_some(left_idx),
            (right_idx < len).then_some(right_idx),
        )
    }

    fn push(&mut self, value: T) {
        self.0.push(value);
        self.up_heapify(self.0.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.0.is_empty() {
            return None;
        }

        let len = self.len();

        if len > 1 {
            self.0.swap(0, len - 1);
        }

        let ret = self.0.pop().unwrap();

        self.down_heapify(0);

        Some(ret)
    }

    fn up_heapify(&mut self, i: usize) {
        if let Some(parent_idx) = self.get_parent_idx(i) {
            if self.0[parent_idx] < self.0[i] {
                self.0.swap(parent_idx, i);
            }

            self.up_heapify(parent_idx);
        }
    }

    fn down_heapify(&mut self, i: usize) {
        let child_idx = match self.get_children_idx(i) {
            (None, None) => return,
            (Some(left_idx), None) => left_idx,
            (None, Some(right_idx)) => right_idx,
            (Some(left_idx), Some(right_idx)) => {
                if self.0[left_idx] > self.0[right_idx] {
                    left_idx
                } else {
                    right_idx
                }
            }
        };

        if self.0[child_idx] > self.0[i] {
            self.0.swap(i, child_idx);
            self.down_heapify(child_idx);
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut arr: Vec<_> = input.collect();

    merge_sort(&mut arr[..], n);

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

    // let mut heap = Heap::new();
    // input.for_each(|num| heap.push(Reverse(num)));
    // // let mut heap: BinaryHeap<_> = input.collect();

    // while let Some(Reverse(num)) = heap.pop() {
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

    arr.copy_from_slice(&temp);
}
