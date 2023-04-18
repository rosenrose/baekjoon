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
        let rest: Vec<_> = (0..numbers).filter(|n| !selected.contains(n)).collect();
        // println!("{selected:?} {rest:?}");
        let (mut start, mut link) = (0, 0);

        for i in 0..selected.len() - 1 {
            for j in i + 1..selected.len() {
                let (a, b) = (selected[i], selected[j]);
                start += matrix[a][b] + matrix[b][a];

                let (a, b) = (rest[i], rest[j]);
                link += matrix[a][b] + matrix[b][a];
            }
        }

        return start.abs_diff(link);
    }

    let takes = if start == 0 {
        1
    } else {
        numbers - selected.len() + 1
    };

    (start..numbers).take(takes).fold(u32::MAX, |diff, num| {
        selected[depth] = num;
        let result = combination_pairs(depth + 1, num + 1, selected, numbers, matrix);

        result.min(diff)
    })
}
