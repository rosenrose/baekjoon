fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, mut k] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);

        let mut arr = parse_int_vec(&buf);
        let mut num = 0;

        merge_sort(&mut arr[..], n as usize, &mut k, &mut num);

        println!("{}", if k > 0 { -1 } else { num });
    }
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
