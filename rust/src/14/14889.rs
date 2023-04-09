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
        let start = combinations(0, 0, &mut vec![0; 2], selected, matrix);
        let link = combinations(0, 0, &mut vec![0; 2], &rest, matrix);

        return start.abs_diff(link);
    }

    (0..numbers)
        .enumerate()
        .skip(start)
        .take(if start == 0 {
            1
        } else {
            numbers - selected.len() + 1
        })
        .fold(u32::MAX, |diff, (i, num)| {
            if selected[..depth].contains(&num) {
                return diff;
            }

            selected[depth] = num;

            let result = combination_pairs(depth + 1, i + 1, selected, numbers, matrix);

            result.min(diff)
        })
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    nums: &Vec<usize>,
    matrix: &Vec<Vec<u32>>,
) -> u32 {
    if depth == selected.len() {
        let (a, b) = (selected[0], selected[1]);
        return matrix[a][b] + matrix[b][a];
    }

    nums.iter()
        .enumerate()
        .skip(start)
        .take(nums.len() - selected.len() + 1)
        .map(|(i, &num)| {
            if selected[..depth].contains(&num) {
                return 0;
            }

            selected[depth] = num;

            combinations(depth + 1, i + 1, selected, nums, matrix)
        })
        .sum()
}
