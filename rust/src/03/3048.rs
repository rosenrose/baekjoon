use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let time: usize = input.next_back().unwrap().parse().unwrap();
    let (ants2, ants1) = (input.next_back().unwrap(), input.next_back().unwrap());

    let mut order: Vec<_> = ants1
        .chars()
        .rev()
        .chain(ants2.chars())
        .enumerate()
        .collect();

    for (i, (pos, _)) in order.iter_mut().enumerate() {
        let original_pos;
        let t = time.min(ants1.len() + ants2.len() - 1);
        let offset;

        if i < ants1.len() {
            original_pos = (ants1.len() - 1) - i;
            offset = t.saturating_sub(original_pos).min(ants2.len());

            *pos += offset;
        } else {
            original_pos = i - ants1.len();
            offset = t.saturating_sub(original_pos).min(ants1.len());

            *pos -= offset;
        }
    }

    order.sort_by_key(|&(pos, _)| pos);

    for (_, ant) in order {
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
