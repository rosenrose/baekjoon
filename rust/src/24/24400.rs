use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, query] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut arr_a: Vec<_> = input.by_ref().take(n).collect();
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

    println!(
        "{}",
        select_nth(&mut arr_a, (0, n), query, &arr_b, &mut count, n) as u8
    );
}

fn select_nth(
    arr_a: &mut Vec<i32>,
    (lo, hi): (usize, usize),
    query: usize,
    arr_b: &[i32],
    count: &mut usize,
    n: usize,
) -> bool {
    if *count == n {
        return true;
    }

    let len = hi - lo;

    if len <= 1 {
        return false;
    }

    let pivot = hi - 1;
    let mut i = lo;

    for j in lo..pivot {
        if arr_a[j] <= arr_a[pivot] {
            if arr_a[i] == arr_b[i] {
                *count -= 1;
            }
            if arr_a[j] == arr_b[j] {
                *count -= 1;
            }

            arr_a.swap(i, j);

            if arr_a[i] == arr_b[i] {
                *count += 1;
            }
            if arr_a[j] == arr_b[j] {
                *count += 1;
            }

            if *count == n {
                return true;
            }

            i += 1;
        }
    }

    if i != pivot {
        if arr_a[i] == arr_b[i] {
            *count -= 1;
        }
        if arr_a[pivot] == arr_b[pivot] {
            *count -= 1;
        }

        arr_a.swap(i, pivot);

        if arr_a[i] == arr_b[i] {
            *count += 1;
        }
        if arr_a[pivot] == arr_b[pivot] {
            *count += 1;
        }

        if *count == n {
            return true;
        }
    }

    let nth = i - lo + 1;

    match query.cmp(&nth) {
        Ordering::Less => select_nth(arr_a, (lo, i), query, arr_b, count, n),
        Ordering::Equal => false,
        Ordering::Greater => select_nth(arr_a, (i + 1, hi), query - nth, arr_b, count, n),
    }
}
