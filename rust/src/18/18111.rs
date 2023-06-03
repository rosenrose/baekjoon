use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, _, b) = (input.next(), input.next(), input.next().unwrap());
    let (mut min, mut max) = (256, 0);
    let mut height_counts = [0; 257];

    for height in input {
        height_counts[height as usize] += 1;
        min = height.min(min);
        max = height.max(max);
    }

    let mut min_time = u32::MAX;
    let mut times = [0; 257];

    for height in min..=max {
        let (mut time, mut blocks) = (0, b);

        for h in min..=max {
            let count = height_counts[h as usize];
            let diff = h.abs_diff(height);

            if h > height {
                time += diff * count * 2;
                blocks += (diff * count) as i32;
            }
            if h < height {
                time += diff * count;
                blocks -= (diff * count) as i32;
            }
        }

        if blocks >= 0 {
            min_time = time.min(min_time);
            times[height as usize] = time;
        }
    }

    let max_height = times
        .iter()
        .enumerate()
        .filter_map(|(height, &time)| (time == min_time).then_some(height))
        .max()
        .unwrap();

    println!("{min_time} {max_height}");
}
