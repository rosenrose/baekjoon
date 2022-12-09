use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();
    input.next();

    let b = input.next().unwrap();
    let (mut min, mut max) = (256, 0);
    let mut height_counts = [0; 257];

    for height in input {
        height_counts[height as usize] += 1;
        min = height.min(min);
        max = height.max(max);
    }

    let mut min_time = u32::MAX;

    let height_times: Vec<_> = (min..=max)
        .filter_map(|height| {
            let (mut time, mut blocks) = (0, b);

            for h in min..=max {
                let c = height_counts[h as usize];
                let diff = h.abs_diff(height);

                if h > height {
                    time += diff * c * 2;
                    blocks += (diff * c) as i32;
                }
                if h < height {
                    time += diff * c;
                    blocks -= (diff * c) as i32;
                }
            }

            (blocks >= 0).then(|| {
                min_time = time.min(min_time);
                (height, time)
            })
        })
        .collect();

    let (height, time) = height_times
        .iter()
        .filter(|(_, t)| *t == min_time)
        .max_by_key(|(h, _)| h)
        .unwrap();

    println!("{time} {height}");
}
