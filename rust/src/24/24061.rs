use std::fmt::Write;
use std::io;

const MAX: usize = 500_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, mut k] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut arr = [0; MAX];

    for (i, num) in input.enumerate() {
        arr[i] = num;
    }

    if merge_sort(&mut arr[..n], &mut k) {
        for num in &arr[..n] {
            write!(output, "{num} ").unwrap();
        }
    } else {
        writeln!(output, "-1").unwrap();
    }

    print!("{output}");
}

fn merge_sort(arr: &mut [i32], k: &mut usize) -> bool {
    let n = arr.len();

    if n <= 1 {
        return false;
    }

    let mid = (n + 1) >> 1;
    let result = merge_sort(&mut arr[..mid], k) || merge_sort(&mut arr[mid..], k);

    if result {
        return result;
    }

    let mut temp = vec![0; n];
    let (mut a, mut b) = (0, mid);

    for i in 0..n {
        if a < mid && b < n {
            if arr[a] < arr[b] {
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

    for (i, &num) in temp.iter().enumerate() {
        arr[i] = num;
        *k -= 1;

        if *k == 0 {
            return true;
        }
    }

    false
}
