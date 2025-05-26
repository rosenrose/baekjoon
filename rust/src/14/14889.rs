use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let n = input.next().unwrap() as usize;
    let mut matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            matrix[r][c] = num;
        }
    }

    let min_diff = combination_pairs(0, 0, &mut [0; MAX / 2][..n / 2], &matrix[..n]);

    println!("{min_diff}");
}

fn combination_pairs(
    depth: usize,
    start: usize,
    selected: &mut [usize],
    matrix: &[[u32; MAX]],
) -> u32 {
    if depth == selected.len() {
        let mut rest = [0; MAX / 2];

        for (i, num) in (0..matrix.len())
            .filter(|n| !selected.contains(n))
            .enumerate()
        {
            rest[i] = num;
        }
        // println!("{selected:?} {rest:?}");
        let (mut start_score, mut link_score) = (0, 0);

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
        return combination_pairs(depth + 1, 1, selected, matrix);
    }

    let takes = matrix.len() - (selected.len() - 1);

    (start..depth + takes)
        .map(|num| {
            selected[depth] = num;
            combination_pairs(depth + 1, num + 1, selected, matrix)
        })
        .min()
        .unwrap()
}
