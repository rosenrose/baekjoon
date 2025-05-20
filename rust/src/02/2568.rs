use std::fmt::Write;
use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut cables = [(0, 0); MAX];

    for (i, [a, b]) in (0..n)
        .map(|_| [(); 2].map(|_| input.next().unwrap()))
        .enumerate()
    {
        cables[i] = (a, b);
    }

    cables[..n].sort_unstable();

    let mut lis_temp = [0; MAX];
    lis_temp[0] = cables[0].1;
    let mut lis_temp_len = 1;

    let mut num_indices = [(0, 0); MAX];
    num_indices[0] = (lis_temp[0], 0);
    let mut num_indices_len = 1;

    for &(_, num) in &cables[1..n] {
        if num > lis_temp[lis_temp_len - 1] {
            num_indices[num_indices_len] = (num, lis_temp_len as i32);
            num_indices_len += 1;

            lis_temp[lis_temp_len] = num;
            lis_temp_len += 1;
            continue;
        }

        let i = lis_temp[..lis_temp_len].partition_point(|&n| n < num);
        lis_temp[i] = num;

        num_indices[num_indices_len] = (num, i as i32);
        num_indices_len += 1;
    }

    let lis_len = lis_temp_len;
    let mut lis = [0; MAX];

    for &(num, idx) in num_indices[..num_indices_len].iter().rev() {
        let idx = idx as usize;

        if idx + 1 == lis_temp_len {
            lis[idx] = num;
            lis_temp_len -= 1;
        }

        if lis_temp_len == 0 {
            break;
        }
    }

    let mut count = 0;

    for (a, _) in cables[..n]
        .iter()
        .filter(|(_, b)| lis[..lis_len].binary_search(b).is_err())
    {
        count += 1;
        writeln!(output, "{a}").unwrap();
    }

    print!("{count}\n{output}");
}
