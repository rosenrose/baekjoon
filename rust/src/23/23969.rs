use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, k] = [(); 2].map(|_| input.next().unwrap());
    let arr: Vec<_> = input.collect();

    if let Some(swapped) = bubble_sort(arr, k) {
        for num in swapped {
            print!("{num} ");
        }
    } else {
        println!("-1");
    }
}

fn bubble_sort(mut arr: Vec<i32>, k: i32) -> Option<Vec<i32>> {
    let n = arr.len();
    let mut count = 0;

    for i in 0..n - 1 {
        for j in 0..n - 1 - i {
            if arr[j] <= arr[j + 1] {
                continue;
            }

            arr.swap(j, j + 1);
            count += 1;

            if count == k {
                return Some(arr);
            }
        }
    }

    None
}
