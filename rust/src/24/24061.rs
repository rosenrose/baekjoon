use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [_, mut k] = [(); 2].map(|_| input.next().unwrap());
    let mut arr: Vec<_> = input.collect();

    if merge_sort(&mut arr, &mut k) {
        for num in arr {
            write!(output, "{num} ").unwrap();
        }
    } else {
        writeln!(output, "-1").unwrap();
    }

    print!("{output}");
}

fn merge_sort(arr: &mut [i32], k: &mut i32) -> bool {
    let n = arr.len();

    if n <= 1 {
        return false;
    }

    let pivot = (n + 1) >> 1;
    let result = merge_sort(&mut arr[..pivot], k) || merge_sort(&mut arr[pivot..], k);

    if result {
        return result;
    }

    let mut temp = vec![0; n];
    let (mut a, mut b) = (0, pivot);

    for i in 0..n {
        if a < pivot && b < n {
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

    for (i, &num) in temp.iter().enumerate() {
        arr[i] = num;
        *k -= 1;

        if *k == 0 {
            return true;
        }
    }

    false
}
