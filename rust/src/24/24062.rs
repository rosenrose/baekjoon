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
        merge_sort(&mut arr_a, (0, n), &arr_b, &mut count, n) as u8
    );
}

fn merge_sort(
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

    let mid = (lo + hi + 1) >> 1;
    let result = merge_sort(arr_a, (lo, mid), arr_b, count, n)
        || merge_sort(arr_a, (mid, hi), arr_b, count, n);

    if result {
        return result;
    }

    let mut temp = vec![0; len];
    let (mut left, mut right) = (lo, mid);

    for i in 0..len {
        if left < mid && right < hi {
            if arr_a[left] < arr_a[right] {
                temp[i] = arr_a[left];
                left += 1;
            } else {
                temp[i] = arr_a[right];
                right += 1;
            }
        } else {
            if left == mid {
                temp[i] = arr_a[right];
                right += 1;
            } else {
                temp[i] = arr_a[left];
                left += 1;
            }
        }
    }

    for (i, &num) in temp.iter().enumerate() {
        if arr_a[i + lo] == arr_b[i + lo] {
            *count -= 1;
        }

        arr_a[i + lo] = num;

        if arr_a[i + lo] == arr_b[i + lo] {
            *count += 1;
        }
        if *count == n {
            return true;
        }
    }

    false
}
