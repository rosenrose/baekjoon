use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let _n = input.next();
    let mut lis_temp = vec![input.next().unwrap()];
    let mut num_indices = vec![(lis_temp[0], 0)];
    let mut lis = Vec::new();

    for num in input {
        if num > *lis_temp.last().unwrap() {
            num_indices.push((num, lis_temp.len()));
            lis_temp.push(num);
            continue;
        }

        let pos = lis_temp.binary_search(&num).unwrap_or_else(|i| i);

        num_indices.push((num, pos));
        lis_temp[pos] = num;
    }

    let mut lis_len = lis_temp.len();

    for &(num, index) in num_indices.iter().rev() {
        if index + 1 == lis_len {
            lis.push(num);
            lis_len -= 1;
        }

        if lis_len == 0 {
            break;
        }
    }

    writeln!(output, "{}", lis.len()).unwrap();

    for num in lis.iter().rev() {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
