use std::collections::BTreeMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut arr_a = Vec::with_capacity(n);
    let indices: BTreeMap<_, _> = input
        .by_ref()
        .take(n)
        .enumerate()
        .map(|(i, num)| {
            arr_a.push(num);
            (num, i as i32)
        })
        .collect();

    let mut count = 0;
    let arr_b: Vec<_> = input
        .enumerate()
        .map(|(i, num)| {
            if arr_a[i] == num {
                count += 1;
            }

            num
        })
        .collect();

    println!("{}", u8::from(selection_sort(arr_a, indices, arr_b, count)));
}

fn selection_sort(
    mut arr_a: Vec<i32>,
    mut indices: BTreeMap<i32, i32>,
    arr_b: Vec<i32>,
    mut count: usize,
) -> bool {
    let n = arr_a.len();

    for i in (1..n).rev() {
        let (max, max_idx) = indices.pop_last().unwrap();

        if max > arr_a[i] {
            indices.insert(arr_a[i], max_idx);
            arr_a.swap(i, max_idx as usize);

            for idx in [i, max_idx as usize] {
                if arr_a[idx] == arr_b[idx] {
                    count += 1;
                }
            }
        }

        if count == n {
            return true;
        }
    }

    false
}
