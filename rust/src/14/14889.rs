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
    matrix: &[Vec<u32>],
) -> u32 {
    if depth == selected.len() {
        let rest: Vec<_> = (0..numbers).filter(|n| !selected.contains(n)).collect();
        let (mut start_score, mut link_score) = (0, 0);
        // println!("{selected:?} {rest:?}");
        for i in 0..selected.len() - 1 {
            for j in i + 1..selected.len() {
                let (a, b) = (selected[i], selected[j]);
                start_score += matrix[a][b] + matrix[b][a];

                let (a, b) = (rest[i], rest[j]);
                link_score += matrix[a][b] + matrix[b][a];
            }
        }

        return start_score.abs_diff(link_score);
    }

    if depth == 0 {
        selected[depth] = 0;
        return combination_pairs(depth + 1, 1, selected, numbers, matrix);
    }

    let takes = numbers - (selected.len() - 1);

    (start..depth + takes)
        .map(|num| {
            selected[depth] = num;
            combination_pairs(depth + 1, num + 1, selected, numbers, matrix)
        })
        .min()
        .unwrap()
}
