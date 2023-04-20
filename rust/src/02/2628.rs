use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);
    let mut input = || input.next().unwrap();

    let (width, height) = (input(), input());
    let (mut horizontals, mut verticals) = (vec![0, height], vec![0, width]);

    for (dir, num) in (0..input()).map(|_| (input(), input())) {
        match dir {
            0 => horizontals.push(num),
            1 => verticals.push(num),
            _ => (),
        }
    }

    horizontals.sort();
    verticals.sort();

    let max_width = (1..verticals.len())
        .map(|i| verticals[i - 1].abs_diff(verticals[i]))
        .max()
        .unwrap();
    let max_height = (1..horizontals.len())
        .map(|i| horizontals[i - 1].abs_diff(horizontals[i]))
        .max()
        .unwrap();

    println!("{}", max_width * max_height);
}
