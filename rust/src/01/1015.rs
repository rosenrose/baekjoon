use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut a: Vec<_> = input.skip(1).enumerate().collect();
    a.sort_by_key(|&(_, num)| num);

    let p = (0..a.len()).flat_map(|i| a.iter().position(|&(idx, _)| idx == i));

    for idx in p {
        print!("{idx} ");
    }
}
