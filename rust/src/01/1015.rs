use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut a = [(0, 0); MAX];

    for (i, num) in input.enumerate() {
        a[i] = (i, num);
    }

    a[..n].sort_by_key(|&(_, num)| num);

    let p = (0..n).flat_map(|i| a[..n].iter().position(|&(idx, _)| idx == i));

    for idx in p {
        print!("{idx} ");
    }
}
