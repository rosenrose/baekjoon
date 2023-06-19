use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, k] = [(); 2].map(|_| input.next().unwrap());
    let arr: Vec<_> = input.collect();

    if let Some(num) = insertion_sort(arr, k) {
        println!("{num}");
    } else {
        println!("-1");
    }
}

fn insertion_sort(mut arr: Vec<i32>, k: i32) -> Option<i32> {
    let mut count = 0;

    for i in 1..arr.len() {
        let num = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > num {
            arr[j] = arr[j - 1];
            count += 1;

            if count == k {
                return Some(arr[j]);
            }

            j -= 1;
        }

        if j != i {
            arr[j] = num;
            count += 1;

            if count == k {
                return Some(arr[j]);
            }
        }
    }

    None
}
