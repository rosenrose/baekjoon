use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut arr: Vec<_> = input.collect();

    merge_sort(&mut arr[..], n as usize);

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

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
    let (mut a, mut b) = (0, pivot);

    for i in 0..len {
        if a < pivot && b < len {
            if arr[a] > arr[b] {
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

    arr.copy_from_slice(&temp[..]);
}
