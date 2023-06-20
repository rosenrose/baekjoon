use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, k] = [(); 2].map(|_| input.next().unwrap());
    let arr: Vec<_> = input.collect();

    if let Some((a, b)) = heap_sort(arr, k) {
        println!("{a} {b}");
    } else {
        println!("-1");
    }
}

fn heap_sort(mut arr: Vec<i32>, mut k: i32) -> Option<(i32, i32)> {
    let n = arr.len();
    let result = build_min_heap(&mut arr, &mut k);

    if result.is_some() {
        return result;
    }

    (1..n).rev().find_map(|i| {
        k -= 1;

        if k == 0 {
            return Some((arr[0], arr[i]));
        }

        arr.swap(0, i);
        heapify(&mut arr, 0, i, &mut k)
    })
}

fn build_min_heap(arr: &mut Vec<i32>, k: &mut i32) -> Option<(i32, i32)> {
    let n = arr.len();

    (0..n / 2).rev().find_map(|i| heapify(arr, i, n, k))
}

fn heapify(arr: &mut Vec<i32>, root: usize, len: usize, k: &mut i32) -> Option<(i32, i32)> {
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
        return None;
    };

    if arr[child_idx] < arr[root] {
        *k -= 1;

        if *k == 0 {
            return Some((arr[child_idx], arr[root]));
        }

        arr.swap(child_idx, root);
        let result = heapify(arr, child_idx, len, k);

        if result.is_some() {
            return result;
        }
    }

    None
}
