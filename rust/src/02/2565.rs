use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut cables = [(0, 0); MAX];

    for (i, [a, b]) in (0..n)
        .map(|_| [(); 2].map(|_| input.next().unwrap()))
        .enumerate()
    {
        cables[i] = (a, b);
    }

    cables[..n].sort();
    // println!("{cables:?}");
    let mut lis_temp = [0; MAX];
    lis_temp[0] = cables[0].1;

    let mut lis_temp_len = 1;

    for &(_, num) in &cables[1..n] {
        if num > lis_temp[lis_temp_len - 1] {
            lis_temp[lis_temp_len] = num;
            lis_temp_len += 1;
            continue;
        }

        let i = lis_temp[..lis_temp_len].partition_point(|&n| n < num);
        lis_temp[i] = num;
    }

    println!("{}", n - lis_temp_len);
}
