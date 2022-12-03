use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let (n, mut k) = (input.next().unwrap(), input.next().unwrap());

    let mut arr: Vec<_> = input.collect();
    let mut num = 0;

    merge_sort(&mut arr[..], n as usize, &mut k, &mut num);

    println!("{}", if k > 0 { -1 } else { num });
}

fn merge_sort(arr: &mut [i32], len: usize, k: &mut i32, num: &mut i32) {
    if len <= 1 {
        return;
    }

    let pivot = len / 2;
    let pivot = if len % 2 == 0 { pivot } else { pivot + 1 };

    merge_sort(&mut arr[..pivot], pivot, k, num);
    merge_sort(&mut arr[pivot..], len - pivot, k, num);

    let mut temp = vec![0; len];
    let (mut a, mut b) = (0, pivot);

    for i in 0..len {
        if a < pivot && b < len {
            if arr[a] < arr[b] {
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

    for (i, &n) in temp.iter().enumerate() {
        arr[i] = n;
        *k -= 1;

        if *k == 0 {
            *num = n;
        }
    }
}
