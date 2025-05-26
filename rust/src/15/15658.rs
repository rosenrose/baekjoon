#[derive(Copy, Clone, Default)]
enum Ops {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
}

use std::io;
use Ops::*;

const NUMS_MAX: usize = 11;
const RESULT_MAX: i32 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; NUMS_MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        nums[i] = num;
    }

    let mut operators = [Default::default(); NUMS_MAX * 4];
    let mut operators_len = 0;

    for op in input.enumerate().flat_map(|(i, count)| {
        std::iter::repeat(match i {
            0 => Add,
            1 => Sub,
            2 => Mul,
            3 => Div,
            _ => unreachable!(),
        })
        .take(count as usize)
    }) {
        operators[operators_len] = op;
        operators_len += 1;
    }

    let (min, max) = permutations(
        0,
        &mut [0; NUMS_MAX - 1][..n - 1],
        &mut [false; NUMS_MAX * 4],
        &operators[..operators_len],
        &nums[..n],
    );

    println!("{max}\n{min}");
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
    visited_idx: &mut [bool],
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

    let mut visited_op = [false; 4];

    operators
        .iter()
        .enumerate()
        .fold((RESULT_MAX, -RESULT_MAX), |(min, max), (i, &op)| {
            if visited_idx[i] || visited_op[op as usize] {
                return (min, max);
            }

            visited_idx[i] = true;
            visited_op[op as usize] = true;
            selected[depth] = i;

            let result = permutations(depth + 1, selected, visited_idx, operators, nums);
            visited_idx[i] = false;

            (result.0.min(min), result.1.max(max))
        })
}
