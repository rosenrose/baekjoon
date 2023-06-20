use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [_, k] = [(); 2].map(|_| input.next().unwrap());
    let mut arr: Vec<_> = input.collect();

    if heap_sort(&mut arr, k) {
        for num in arr {
            write!(output, "{num} ").unwrap();
        }
    } else {
        writeln!(output, "-1").unwrap();
    }

    print!("{output}");
}

fn heap_sort(arr: &mut Vec<i32>, mut k: i32) -> bool {
    let n = arr.len();

    if build_min_heap(arr, &mut k) {
        return true;
    }

    (1..n).rev().any(|i| {
        arr.swap(0, i);
        k -= 1;

        if k == 0 {
            return true;
        }

        heapify(arr, 0, i, &mut k)
    })
}

fn build_min_heap(arr: &mut Vec<i32>, k: &mut i32) -> bool {
    let n = arr.len();

    (0..n / 2).rev().any(|i| heapify(arr, i, n, k))
}

fn heapify(arr: &mut Vec<i32>, root: usize, len: usize, k: &mut i32) -> bool {
    let double = (root + 1) << 1;
    let (left, right) = (double - 1, double);

    let child_idx = if right < len {
        if arr[left] < arr[right] {
            left
        } else {
            right
        }
    } else if left < len {
        left
    } else {
        return false;
    };

    if arr[child_idx] < arr[root] {
        arr.swap(child_idx, root);
        *k -= 1;

        if *k == 0 {
            return true;
        }

        if heapify(arr, child_idx, len, k) {
            return true;
        }
    }

    false
}
