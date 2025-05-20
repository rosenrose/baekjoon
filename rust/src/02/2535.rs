use std::io;

const MAX: usize = 100 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut medal_counts = [0; MAX];
    let mut winners = 0;
    let mut infos = [[0; 3]; MAX];

    for (i, info) in (0..n)
        .map(|_| [(); 3].map(|_| input.next().unwrap()))
        .enumerate()
    {
        infos[i] = info;
    }

    infos[..n].sort_by_key(|&[.., score]| score);

    for &[country, student, _] in infos[..n].iter().rev() {
        if medal_counts[country] == 2 {
            continue;
        }

        medal_counts[country] += 1;

        println!("{country} {student}");

        winners += 1;

        if winners == 3 {
            break;
        }
    }
}
