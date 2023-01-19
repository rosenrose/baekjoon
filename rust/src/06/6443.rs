use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let mut word = input.as_bytes().to_vec();
        word.sort();

        let len = word.len();

        loop {
            writeln!(output, "{}", String::from_utf8(word.clone()).unwrap()).unwrap();

            let Some(i) = (1..len).rfind(|&i| word[i - 1] < word[i]) else {
                break;
            };
            let j = (i..len).rfind(|&j| word[j] > word[i - 1]).unwrap();

            word.swap(i - 1, j);
            quick_sort(&mut word[i..], len - i);
        }
    }

    print!("{output}");
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
