use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut arr = [0; MAX];

    for (i, num) in input.enumerate() {
        arr[i] = num;
    }

    // merge_sort(&mut arr[..n]);
    arr[..n].sort_unstable_by_key(|&num| std::cmp::Reverse(num));

    for num in &arr[..n] {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len >> 1;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut temp = vec![0; len];
    let (mut a, mut b) = (0, mid);

    for i in 0..len {
        if a < mid && b < len {
            if arr[a] > arr[b] {
                temp[i] = arr[a];
                a += 1;
            } else {
                temp[i] = arr[b];
                b += 1;
            }
        } else {
            if a == mid {
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
