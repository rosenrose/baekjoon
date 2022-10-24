fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    let mut arr: Vec<i32> = (0..n)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .collect();

    // bubble_sort(&mut arr);
    // selection_sort(&mut arr);
    // insertion_sort(&mut arr);
    quick_sort(&mut arr[..], n);

    for num in arr {
        println!("{num}");
    }
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if arr[j] < arr[j + 1] {
                continue;
            }

            arr.swap(j, j + 1);
        }
    }
}

fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len - 1 {
        let (min_index, _) = arr[i..].iter().enumerate().min_by_key(|(_, &n)| n).unwrap();

        arr.swap(i, min_index + i);
    }
}

fn insertion_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 1..len {
        for j in (0..=i - 1).rev() {
            if arr[j] < arr[j + 1] {
                break;
            }

            arr.swap(j, j + 1);
        }
    }
}

fn quick_sort(arr: &mut [i32], len: i32) {
    if len <= 1 {
        return;
    }

    let mut i = 0;
    let mut j = len - 1;
    let pivot_value = arr[(len / 2) as usize];

    while i <= j {
        while arr[i as usize] < pivot_value {
            i += 1;
        }
        while arr[j as usize] > pivot_value {
            j -= 1;
        }

        if i > j {
            break;
        }

        arr.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }

    quick_sort(&mut arr[..=j as usize], j + 1);
    quick_sort(&mut arr[i as usize..], len - i);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
