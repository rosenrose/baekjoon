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

    println!("{}", u8::from(insertion_sort(arr_a, arr_b, count)));
}

fn insertion_sort(mut arr_a: Vec<i32>, arr_b: Vec<i32>, mut count: usize) -> bool {
    let n = arr_a.len();

    if count == n {
        return true;
    }

    for i in 1..n {
        let num = arr_a[i];
        let mut j = i;

        while j > 0 && arr_a[j - 1] > num {
            arr_a[j] = arr_a[j - 1];

            if arr_a[j] == arr_b[j] {
                count += 1;
            }
            if count == n {
                return true;
            }

            j -= 1;
        }

        if j != i {
            arr_a[j] = num;

            if arr_a[j] == arr_b[j] {
                count += 1;
            }
            if count == n {
                return true;
            }
        }
    }

    false
}
