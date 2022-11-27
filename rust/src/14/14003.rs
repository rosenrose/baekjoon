use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut a = buf
        .lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut lis_len_arr = vec![a.next().unwrap()];
    let mut index_arr = vec![(lis_len_arr[0], 0)];
    let mut lis = Vec::new();

    for num in a {
        if num > *lis_len_arr.last().unwrap() {
            index_arr.push((num, lis_len_arr.len()));
            lis_len_arr.push(num);
            continue;
        }

        let pos = match lis_len_arr.binary_search(&num) {
            Ok(i) => i,
            Err(i) => i,
        };

        index_arr.push((num, pos));
        lis_len_arr[pos] = num;
    }

    for &(num, index) in index_arr.iter().rev() {
        if index + 1 == lis_len_arr.len() {
            lis.push(num);
            lis_len_arr.pop();
        }

        if lis_len_arr.is_empty() {
            break;
        }
    }

    writeln!(stdout, "{}", lis.len()).unwrap();

    for num in lis.iter().rev() {
        write!(stdout, "{num} ").unwrap();
    }
}
