use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let mut arr: Vec<_> = input.collect();

    // bubble_sort(&mut arr);
    // selection_sort(&mut arr);
    // insertion_sort(&mut arr);
    quick_sort(&mut arr, n as usize);

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len - 1 {
        for j in 1..len - i {
            if arr[j - 1] < arr[j] {
                continue;
            }

            arr.swap(j - 1, j);
        }
    }
}

fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len - 1 {
        let (min_index, _) = arr[i..].iter().enumerate().min_by_key(|(_, &n)| n).unwrap();

        arr.swap(i, min_index + i);
    }
}

fn insertion_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 1..len {
        for j in (1..=i).rev() {
            if arr[j - 1] < arr[j] {
                break;
            }

            arr.swap(j - 1, j);
        }
    }
}

fn quick_sort(arr: &mut [i32], len: usize) {
    if len <= 1 {
        return;
    }

    let (mut i, mut j) = (0, len - 1);
    let pivot_value = arr[len / 2];

    while i <= j {
        while arr[i] < pivot_value {
            i += 1;
        }
        while arr[j] > pivot_value {
            j -= 1;
        }

        if i > j {
            break;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }

    quick_sort(&mut arr[..=j], j + 1);
    quick_sort(&mut arr[i..], len - i);
}
