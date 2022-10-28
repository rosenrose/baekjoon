use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut arr: Vec<i32> = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();

            buf.trim().parse().unwrap()
        })
        .collect();

    // bubble_sort(&mut arr);
    // selection_sort(&mut arr);
    // insertion_sort(&mut arr);
    quick_sort(&mut arr[..], n);

    for num in arr {
        writeln!(stdout, "{num}").unwrap();
    }
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if arr[j] < arr[j + 1] {
                continue;
            }

            arr.swap(j, j + 1);
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
        for j in (0..=i - 1).rev() {
            if arr[j] < arr[j + 1] {
                break;
            }

            arr.swap(j, j + 1);
        }
    }
}

fn quick_sort(arr: &mut [i32], len: i32) {
    if len <= 1 {
        return;
    }

    let mut i = 0;
    let mut j = len - 1;
    let pivot_value = arr[(len / 2) as usize];

    while i <= j {
        while arr[i as usize] < pivot_value {
            i += 1;
        }
        while arr[j as usize] > pivot_value {
            j -= 1;
        }

        if i > j {
            break;
        }

        arr.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }

    quick_sort(&mut arr[..=j as usize], j + 1);
    quick_sort(&mut arr[i as usize..], len - i);
}
