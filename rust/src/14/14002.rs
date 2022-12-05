use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut a = buf
        .split_ascii_whitespace()
        .skip(1)
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

        let pos = lis_len_arr.binary_search(&num).unwrap_or_else(|i| i);

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

    println!("{}", lis.len());

    for num in lis.iter().rev() {
        print!("{num} ");
    }
}
