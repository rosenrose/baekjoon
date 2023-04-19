use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let (mut houses, mut chickens) = (Vec::new(), Vec::new());

    for r in 0..n {
        for c in 0..n {
            match input.next() {
                Some(1) => houses.push((r, c)),
                Some(2) => chickens.push((r, c)),
                _ => (),
            }
        }
    }

    let min_dist = (1..=m)
        .map(|i| combinations(0, 0, &mut vec![(0, 0); i], &chickens, &houses))
        .min()
        .unwrap();

    println!("{min_dist}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<(usize, usize)>,
    chickens: &Vec<(usize, usize)>,
    houses: &Vec<(usize, usize)>,
) -> usize {
    if depth == selected.len() {
        let chicken_dist = houses
            .iter()
            .map(|&(house_r, house_c)| {
                selected
                    .iter()
                    .map(|(chicken_r, chicken_c)| {
                        chicken_r.abs_diff(house_r) + chicken_c.abs_diff(house_c)
                    })
                    .min()
                    .unwrap()
            })
            .sum();

        return chicken_dist;
    }

    let takes = chickens.len() - selected.len() + 1;

    (start..depth + takes)
        .map(|i| {
            selected[depth] = chickens[i];
            combinations(depth + 1, i + 1, selected, chickens, houses)
        })
        .min()
        .unwrap()
}
