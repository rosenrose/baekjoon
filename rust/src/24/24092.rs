use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
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
        u8::from(quick_sort(&mut arr_a, (0, n), &arr_b, &mut count, n))
    );
}

fn quick_sort(
    arr_a: &mut Vec<i32>,
    (lo, hi): (usize, usize),
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
            for idx in [i, j] {
                if arr_a[idx] == arr_b[idx] {
                    *count -= 1;
                }
            }

            arr_a.swap(i, j);

            for idx in [i, j] {
                if arr_a[idx] == arr_b[idx] {
                    *count += 1;
                }
            }

            if *count == n {
                return true;
            }

            i += 1;
        }
    }

    if i != pivot {
        for idx in [i, pivot] {
            if arr_a[idx] == arr_b[idx] {
                *count -= 1;
            }
        }

        arr_a.swap(i, pivot);

        for idx in [i, pivot] {
            if arr_a[idx] == arr_b[idx] {
                *count += 1;
            }
        }

        if *count == n {
            return true;
        }
    }

    quick_sort(arr_a, (lo, i), arr_b, count, n) || quick_sort(arr_a, (i + 1, hi), arr_b, count, n)
}
