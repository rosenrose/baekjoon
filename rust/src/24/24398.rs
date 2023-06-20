use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, query, mut k] = [(); 3].map(|_| input.next().unwrap());
    let mut arr: Vec<_> = input.collect();

    if let Some((a, b)) = select_nth(&mut arr, query as usize, &mut k) {
        println!("{a} {b}");
    } else {
        println!("-1");
    }
}

fn select_nth(arr: &mut [i32], query: usize, k: &mut i32) -> Option<(i32, i32)> {
    let len = arr.len();

    if len <= 1 {
        return None;
    }

    let pivot = len - 1;
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            *k -= 1;

            if *k == 0 {
                return Some((arr[i], arr[j]));
            }

            i += 1;
        }
    }

    if i != pivot {
        arr.swap(i, pivot);
        *k -= 1;

        if *k == 0 {
            return Some((arr[i], arr[pivot]));
        }
    }

    let nth = i + 1;

    match query.cmp(&nth) {
        Ordering::Less => select_nth(&mut arr[..i], query, k),
        Ordering::Equal => None,
        Ordering::Greater => select_nth(&mut arr[i + 1..], query - nth, k),
    }
}
