use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (height, width, k) = (input() as usize, input() as usize, input() as usize);
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| (0..width).map(|_| input()).collect())
        .collect();

    let max_sum = combinations(0, 0, &mut vec![(0, 0); k], &map);

    println!("{max_sum}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<(usize, usize)>,
    map: &[Vec<i32>],
) -> i32 {
    if depth == selected.len() {
        return selected.iter().map(|&(r, c)| map[r][c]).sum();
    }

    let (width, height) = (map[0].len(), map.len());
    let takes = (width * height) - selected.len() + 1;

    (start..depth + takes)
        .map(|i| {
            let (r, c) = (i / width, i % width);
            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(height - 1), c),
                (r, (c + 1).min(width - 1)),
            ];

            if adjacents.iter().any(|adj| selected[..depth].contains(adj)) {
                return i32::MIN;
            }

            selected[depth] = (r, c);
            combinations(depth + 1, i + 1, selected, map)
        })
        .max()
        .unwrap()
}
