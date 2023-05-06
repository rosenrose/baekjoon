#[derive(Copy, Clone)]
enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

use std::io;
use Ops::{Add, Div, Mul, Sub};

const MAX: i32 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums: Vec<_> = input.by_ref().take(n).collect();
    let operators: Vec<_> = input
        .enumerate()
        .flat_map(|(i, count)| {
            std::iter::repeat(match i {
                0 => Add,
                1 => Sub,
                2 => Mul,
                3 => Div,
                _ => unreachable!(),
            })
            .take(count as usize)
        })
        .collect();

    let (min, max) = permutations(0, &mut vec![0; n - 1], &operators, &nums);

    println!("{max}\n{min}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    operators: &[Ops],
    nums: &[i32],
) -> (i32, i32) {
    if depth == selected.len() {
        let result = selected.iter().enumerate().fold(nums[0], |acc, (i, &sel)| {
            let num = nums[i + 1];

            match operators[sel] {
                Add => acc + num,
                Sub => acc - num,
                Mul => acc * num,
                Div => acc / num,
            }
        });

        return (result, result);
    }

    let mut visited = [false; 4];

    operators
        .iter()
        .enumerate()
        .fold((MAX, -MAX), |(min, max), (i, &op)| {
            if visited[op as usize] || selected[..depth].contains(&i) {
                return (min, max);
            }

            visited[op as usize] = true;
            selected[depth] = i;

            let result = permutations(depth + 1, selected, operators, nums);

            (result.0.min(min), result.1.max(max))
        })
}
