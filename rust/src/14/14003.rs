use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut lis_temp = vec![input.by_ref().skip(1).next().unwrap()];
    let mut num_indices = vec![(lis_temp[0], 0)];

    for num in input {
        if num > *lis_temp.last().unwrap() {
            num_indices.push((num, lis_temp.len() as i32));
            lis_temp.push(num);
            continue;
        }

        let i = lis_temp.partition_point(|&n| n < num);
        lis_temp[i] = num;
        num_indices.push((num, i as i32));
    }

    let mut lis_len = lis_temp.len();
    let mut lis = vec![0; lis_len];

    for &(num, idx) in num_indices.iter().rev() {
        let idx = idx as usize;

        if idx + 1 == lis_len {
            lis[idx] = num;
            lis_len -= 1;
        }

        if lis_len == 0 {
            break;
        }
    }

    writeln!(output, "{}", lis.len()).unwrap();

    for num in lis {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
