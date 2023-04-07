use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (_n1, _n2) = (input(), input());
    let (ants1, ants2) = (input(), input());
    let time: usize = input().parse().unwrap();

    let mut idx_ants: Vec<_> = ants1
        .chars()
        .rev()
        .chain(ants2.chars())
        .enumerate()
        .collect();

    for (i, (idx, _)) in idx_ants.iter_mut().enumerate() {
        let original_idx;
        let offset;
        let t = time.min(ants1.len() + ants2.len() - 1);

        if i < ants1.len() {
            original_idx = (ants1.len() - 1) - i;
            offset = t.saturating_sub(original_idx).min(ants2.len());

            *idx += offset;
        } else {
            original_idx = i - ants1.len();
            offset = t.saturating_sub(original_idx).min(ants1.len());

            *idx -= offset;
        }
    }

    idx_ants.sort();

    for (_, ant) in idx_ants {
        print!("{ant}");
    }
}

/*
0  CBADEFGH
1  CBDAEFGH
2  CDBEAFGH
3  DCEBFAGH
4  DECFBGAH
5  DEFCGBHA
6  DEFGCHBA
7  DEFGHCBA
*/
