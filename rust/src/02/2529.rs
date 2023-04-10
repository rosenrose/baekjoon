use std::cmp::Ordering::{self, Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let k: usize = input.next().unwrap().parse().unwrap();
    let operators: Vec<_> = input
        .map(|op| match op {
            "<" => Less,
            ">" => Greater,
            _ => Ordering::Equal,
        })
        .collect();

    let (min, max) = permutations(0, &mut vec![0; k + 1], &operators);

    println!("{max:0digits$}\n{min:0digits$}", digits = k + 1);
}

fn permutations(depth: usize, selected: &mut Vec<i64>, operators: &Vec<Ordering>) -> (i64, i64) {
    if depth == selected.len() {
        let result = selected.iter().fold(0, |acc, num| acc * 10 + num);

        return (result, result);
    }

    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .iter()
        .fold((9_876_543_210, 1), |(min, max), &num| {
            if selected[..depth].contains(&num) {
                return (min, max);
            }
            if depth > 0 && (selected[depth - 1].cmp(&num) != operators[depth - 1]) {
                return (min, max);
            }

            selected[depth] = num;

            let result = permutations(depth + 1, selected, operators);

            (result.0.min(min), result.1.max(max))
        })
}
