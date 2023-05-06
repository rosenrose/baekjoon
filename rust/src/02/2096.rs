use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut min_maxes = [(0, 0); 3];

    for _ in 0..input.next().unwrap() {
        let mut next = min_maxes;
        // println!("{min_maxes:?}");
        for (i, num) in input.by_ref().take(3).enumerate() {
            next[i] = match i {
                0 => (
                    (0..=1).map(|j| min_maxes[j].0 + num).min().unwrap(),
                    (0..=1).map(|j| min_maxes[j].1 + num).max().unwrap(),
                ),
                1 => (
                    (0..=2).map(|j| min_maxes[j].0 + num).min().unwrap(),
                    (0..=2).map(|j| min_maxes[j].1 + num).max().unwrap(),
                ),
                2 => (
                    (1..=2).map(|j| min_maxes[j].0 + num).min().unwrap(),
                    (1..=2).map(|j| min_maxes[j].1 + num).max().unwrap(),
                ),
                _ => unreachable!(),
            }
        }

        min_maxes = next;
    }
    // println!("{min_maxes:?}");
    let (min, _) = min_maxes.iter().min_by_key(|(min, _)| min).unwrap();
    let (_, max) = min_maxes.iter().max_by_key(|(_, max)| max).unwrap();

    println!("{max} {min}");
}
