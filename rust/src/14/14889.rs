use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let numbers: Vec<_> = (0..n).collect();
    let matrix: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(n as usize).collect())
        .collect();

    let mut min_diff = u32::MAX;
    combination_pairs(&numbers, n / 2, 0, &mut Vec::new(), &mut min_diff, &matrix);

    println!("{min_diff}");
}

fn combination_pairs(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<i32>,
    min_diff: &mut u32,
    matrix: &Vec<Vec<i32>>,
) {
    if m == 0 {
        let rest = nums
            .iter()
            .copied()
            .filter(|n| !selected.contains(n))
            .collect();
        let (mut start, mut link) = (0, 0);

        combinations(selected, 2, 0, &mut Vec::new(), &mut start, matrix);
        combinations(&rest, 2, 0, &mut Vec::new(), &mut link, matrix);

        *min_diff = start.abs_diff(link).min(*min_diff);

        return;
    }

    nums.iter()
        .enumerate()
        .skip(start)
        .take(if start == 0 {
            1
        } else {
            nums.len() - m as usize + 1
        })
        .for_each(|(i, &num)| {
            if selected.contains(&num) {
                return;
            }

            selected.push(num);

            combination_pairs(nums, m - 1, i + 1, selected, min_diff, matrix);

            selected.pop();
        })
}

fn combinations(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<i32>,
    score: &mut i32,
    matrix: &Vec<Vec<i32>>,
) {
    if m == 0 {
        let (a, b) = (selected[0] as usize, selected[1] as usize);
        *score += matrix[a][b] + matrix[b][a];

        return;
    }

    nums.iter()
        .enumerate()
        .skip(start)
        .take(nums.len() - m as usize + 1)
        .for_each(|(i, &num)| {
            if selected.contains(&num) {
                return;
            }

            selected.push(num);

            combinations(nums, m - 1, i + 1, selected, score, matrix);

            selected.pop();
        });
}
