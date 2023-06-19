use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, k] = [(); 2].map(|_| input.next().unwrap());
    let arr: Vec<_> = input.collect();

    if let Some(swapped) = selection_sort(arr, k) {
        for num in swapped {
            print!("{num} ");
        }
    } else {
        println!("-1");
    }
}

fn selection_sort(mut arr: Vec<i32>, k: i32) -> Option<Vec<i32>> {
    let mut count = 0;

    for i in (1..arr.len()).rev() {
        let (max_idx, &max) = arr[..i]
            .iter()
            .enumerate()
            .max_by_key(|(_, &num)| num)
            .unwrap();

        if max > arr[i] {
            arr.swap(i, max_idx);
            count += 1;

            if count == k {
                return Some(arr);
            }
        }
    }

    None
}
