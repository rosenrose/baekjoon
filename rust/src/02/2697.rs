use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for word in buf.lines().skip(1) {
        if let Some(next) = next_permutation(word.as_bytes().to_vec()) {
            println!("{}", String::from_utf8(next).unwrap());
        } else {
            println!("BIGGEST");
        }
    }
}

fn next_permutation(mut chars: Vec<u8>) -> Option<Vec<u8>> {
    let len = chars.len();

    let i = (1..len).rfind(|&i| chars[i - 1] < chars[i])?;
    let j = (i..len).rfind(|&j| chars[j] > chars[i - 1]).unwrap();

    chars.swap(i - 1, j);
    quick_sort(&mut chars[i..], len - i);

    Some(chars)
}

fn quick_sort(arr: &mut [u8], len: usize) {
    if len <= 1 {
        return;
    }

    let (mut i, mut j) = (0, len - 1);
    let pivot = arr[(len / 2)];

    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }

        if i > j {
            break;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }

    quick_sort(&mut arr[..=j], j + 1);
    quick_sort(&mut arr[i..], len - i);
}
