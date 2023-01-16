use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut medal_counts = vec![0; n + 1];
    let mut winners = 0;

    let mut infos: Vec<_> = (0..n).map(|_| (input(), input(), input())).collect();
    infos.sort_by_key(|&(_, _, score)| score);

    for &(country, student, _) in infos.iter().rev() {
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
