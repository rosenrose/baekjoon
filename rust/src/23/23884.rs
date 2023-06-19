use std::collections::BTreeMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut arr = Vec::with_capacity(n as usize);
    let indices: BTreeMap<_, _> = input
        .enumerate()
        .map(|(i, num)| {
            arr.push(num);
            (num, i as i32)
        })
        .collect();

    if let Some(swapped) = selection_sort(arr, indices, k) {
        for num in swapped {
            write!(output, "{num} ").unwrap();
        }
    } else {
        writeln!(output, "-1").unwrap();
    }

    print!("{output}")
}

fn selection_sort(mut arr: Vec<i32>, mut indices: BTreeMap<i32, i32>, k: i32) -> Option<Vec<i32>> {
    let mut count = 0;

    for i in (1..arr.len()).rev() {
        let (max, max_idx) = indices.pop_last().unwrap();

        if max > arr[i] {
            indices.insert(arr[i], max_idx);
            arr.swap(i, max_idx as usize);
            count += 1;

            if count == k {
                return Some(arr);
            }
        }
    }

    None
}
