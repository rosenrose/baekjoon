use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let arr_a: Vec<_> = input.by_ref().take(n).collect();
    let mut count = 0;
    let arr_b: Vec<_> = input
        .enumerate()
        .map(|(i, num)| {
            if arr_a[i] == num {
                count += 1;
            }

            num
        })
        .collect();

    println!("{}", selection_sort(arr_a, arr_b, count) as u8);
}

fn selection_sort(mut arr_a: Vec<i32>, arr_b: Vec<i32>, mut count: usize) -> bool {
    let n = arr_a.len();

    for i in (1..n).rev() {
        let (max_idx, &max) = arr_a[..i]
            .iter()
            .enumerate()
            .max_by_key(|(_, &num)| num)
            .unwrap();

        if max > arr_a[i] {
            arr_a.swap(i, max_idx);

            for idx in [i, max_idx] {
                if arr_a[idx] == arr_b[idx] {
                    count += 1;
                }
            }
        }

        if count == n {
            return true;
        }
    }

    false
}
