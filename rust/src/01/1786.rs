use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(str::as_bytes);
    let mut output = String::new();

    let [t, p] = [(); 2].map(|_| input.next().unwrap());
    let (count, indices) = kmp_with_index(t, p);

    writeln!(output, "{count}").unwrap();

    for i in indices {
        write!(output, "{} ", i + 1).unwrap();
    }

    print!("{output}");
}

fn kmp_with_index(source: &[u8], target: &[u8]) -> (i32, Vec<usize>) {
    let mut partial_match = vec![0; target.len()];
    let mut i = 0;

    for j in 1..target.len() {
        while (target[i] != target[j]) && i > 0 {
            i = partial_match[i - 1];
        }

        if target[i] == target[j] {
            i += 1;
            partial_match[j] = i;
        }
    }
    // println!("{partial_match:?}");
    i = 0;
    let mut count = 0;
    let mut indices = Vec::new();

    for (idx, &s) in source.iter().enumerate() {
        while target[i] != s && i > 0 {
            i = partial_match[i - 1];
        }

        if target[i] == s {
            if i < target.len() - 1 {
                i += 1;
            } else {
                count += 1;
                indices.push(idx + 1 - target.len());

                i = partial_match[i];
            }
        }
    }

    (count, indices)
}
