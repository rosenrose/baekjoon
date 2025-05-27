use std::io;

const SIZE_MAX: usize = 50;
const HOUSE_MAX: usize = SIZE_MAX * 2;
const CHICKEN_MAX: usize = 13;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut houses = [(0, 0); HOUSE_MAX];
    let mut chickens = [(0, 0); CHICKEN_MAX];
    let (mut houses_len, mut chickens_len) = (0, 0);

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            match num {
                1 => {
                    houses[houses_len] = (r, c);
                    houses_len += 1;
                }
                2 => {
                    chickens[chickens_len] = (r, c);
                    chickens_len += 1;
                }
                _ => (),
            }
        }
    }

    let min_dist = (1..=m)
        .map(|i| {
            combinations(
                0,
                0,
                &mut [(0, 0); CHICKEN_MAX][..i],
                &chickens[..chickens_len],
                &houses[..houses_len],
            )
        })
        .min()
        .unwrap();

    println!("{min_dist}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [(usize, usize)],
    chickens: &[(usize, usize)],
    houses: &[(usize, usize)],
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

    let takes = chickens.len() - (selected.len() - 1);

    (start..depth + takes)
        .map(|i| {
            selected[depth] = chickens[i];
            combinations(depth + 1, i + 1, selected, chickens, houses)
        })
        .min()
        .unwrap()
}
