use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);
    let mut input = || input.next().unwrap();

    let (width, height) = (input(), input());
    let (mut horizontals, mut verticals) = ([0; HEIGHT_MAX - 1], [0; WIDTH_MAX - 1]);
    let (mut horizontals_len, mut verticals_len) = (1, 1);

    horizontals[horizontals_len] = height;
    horizontals_len += 1;
    verticals[verticals_len] = width;
    verticals_len += 1;

    for (dir, num) in (0..input()).map(|_| (input(), input())) {
        match dir {
            0 => {
                horizontals[horizontals_len] = num;
                horizontals_len += 1;
            }
            1 => {
                verticals[verticals_len] = num;
                verticals_len += 1;
            }
            _ => unreachable!(),
        }
    }

    horizontals[..horizontals_len].sort();
    verticals[..verticals_len].sort();

    let max_width = (1..verticals_len)
        .map(|i| verticals[i - 1].abs_diff(verticals[i]))
        .max()
        .unwrap();
    let max_height = (1..horizontals_len)
        .map(|i| horizontals[i - 1].abs_diff(horizontals[i]))
        .max()
        .unwrap();

    println!("{}", max_width * max_height);
}
