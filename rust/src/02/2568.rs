use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut cables: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
    cables.sort_unstable();

    let mut lis_temp = vec![cables[0].1];
    let mut num_indices = vec![(lis_temp[0], 0)];

    for &(_, num) in &cables[1..] {
        if num > *lis_temp.last().unwrap() {
            num_indices.push((num, lis_temp.len()));
            lis_temp.push(num);
            continue;
        }

        let i = lis_temp.partition_point(|&n| n < num);
        lis_temp[i] = num;
        num_indices.push((num, i));
    }

    let mut lis_len = lis_temp.len();
    let mut lis = vec![0; lis_len];

    for &(num, index) in num_indices.iter().rev() {
        if index + 1 == lis_len {
            lis[index] = num;
            lis_len -= 1;
        }

        if lis_len == 0 {
            break;
        }
    }

    let mut count = 0;

    for (a, _) in cables.iter().filter(|(_, b)| !lis.binary_search(b).is_ok()) {
        count += 1;
        writeln!(output, "{a}").unwrap();
    }

    print!("{count}\n{output}");
}
