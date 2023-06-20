use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, mut k] = [(); 2].map(|_| input.next().unwrap());
    let mut arr: Vec<_> = input.collect();

    if quick_sort(&mut arr, &mut k) {
        for num in arr {
            print!("{num} ");
        }
    } else {
        println!("-1");
    }
}

fn quick_sort(arr: &mut [i32], k: &mut i32) -> bool {
    let n = arr.len();

    if n <= 1 {
        return false;
    }

    let pivot = n - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            *k -= 1;

            if *k == 0 {
                return true;
            }

            i += 1;
        }
    }

    if i != pivot {
        arr.swap(i, pivot);
        *k -= 1;

        if *k == 0 {
            return true;
        }
    }

    quick_sort(&mut arr[..i], k) || quick_sort(&mut arr[i + 1..], k)
}
