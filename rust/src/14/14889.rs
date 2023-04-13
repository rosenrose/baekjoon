// use std::io;

// fn main() {
//     let buf = io::read_to_string(io::stdin()).unwrap();
//     let input = buf.lines().flat_map(str::parse::<i32>);

//     let mut nums: Vec<_> = input.skip(1).collect();
//     nums.sort_unstable();

//     if nums.len() == 1 {
//         println!("0");
//         return;
//     }

//     let mut merged = nums[0] + nums[1];
//     let mut count = merged;

//     for i in 2..nums.len() {
//         count += merged + nums[i];
//         merged += nums[i];
//     }

//     println!("{count}");
// }

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let n = input.next().unwrap() as usize;
    let matrix: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    let min_diff = combination_pairs(0, 0, &mut vec![0; n / 2], n, &matrix);

    println!("{min_diff}");
}

fn combination_pairs(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    numbers: usize,
    matrix: &Vec<Vec<u32>>,
) -> u32 {
    if depth == selected.len() {
        let rest = (0..numbers).filter(|n| !selected.contains(n)).collect();
        // println!("{selected:?} {rest:?}");
        let start = combinations(0, 0, &mut [0; 2], selected, matrix);
        let link = combinations(0, 0, &mut [0; 2], &rest, matrix);

        return start.abs_diff(link);
    }

    let takes = if start == 0 {
        1
    } else {
        numbers - selected.len() + 1
    };

    (start..numbers).take(takes).fold(u32::MAX, |diff, num| {
        if selected[..depth].contains(&num) {
            return diff;
        }

        selected[depth] = num;

        let result = combination_pairs(depth + 1, num + 1, selected, numbers, matrix);

        result.min(diff)
    })
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize; 2],
    nums: &Vec<usize>,
    matrix: &Vec<Vec<u32>>,
) -> u32 {
    if depth == selected.len() {
        let [a, b] = &selected;
        return matrix[*a][*b] + matrix[*b][*a];
    }

    let takes = nums.len() - selected.len() + 1;

    nums.iter()
        .enumerate()
        .skip(start)
        .take(takes)
        .map(|(i, &num)| {
            if selected[..depth].contains(&num) {
                return 0;
            }

            selected[depth] = num;

            combinations(depth + 1, i + 1, selected, nums, matrix)
        })
        .sum()
}
