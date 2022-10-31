use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

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

    merge_sort(&mut arr[..], n as usize);

    for num in arr {
        writeln!(stdout, "{num}").unwrap();
    }
}

fn merge_sort(arr: &mut [i32], len: usize) {
    if len <= 1 {
        return;
    }

    let pivot = (len / 2) as usize;

    merge_sort(&mut arr[..pivot], pivot);
    merge_sort(&mut arr[pivot..], len - pivot);

    let mut temp = vec![0; len];
    let mut a = 0;
    let mut b = pivot;

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

    for (i, num) in temp.iter().enumerate() {
        arr[i] = *num;
    }
}
