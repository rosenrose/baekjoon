use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input() as usize;
    let mut eggs: Vec<_> = (0..n).map(|_| (input(), input())).collect();
    let max_count = product(0, &mut eggs, 0);

    println!("{}", max_count.unwrap_or(0));
}

fn product(depth: usize, eggs: &mut Vec<(i32, i32)>, count: usize) -> Option<usize> {
    if depth == eggs.len() {
        return Some(count);
    }

    (0..eggs.len())
        .flat_map(|other_egg| {
            if other_egg == depth {
                return None;
            }

            let is_breakable = eggs[depth].0 > 0 && eggs[other_egg].0 > 0;
            let mut brokens = 0;

            if is_breakable {
                eggs[depth].0 -= eggs[other_egg].1;
                eggs[other_egg].0 -= eggs[depth].1;

                brokens = [eggs[depth].0, eggs[other_egg].0]
                    .iter()
                    .filter(|&&durability| durability <= 0)
                    .count();
            }

            let result = product(depth + 1, eggs, count + brokens);

            if is_breakable {
                eggs[depth].0 += eggs[other_egg].1;
                eggs[other_egg].0 += eggs[depth].1;
            }

            result
        })
        .max()
}
