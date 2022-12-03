use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, _, b] = parse_int_vec(&buf)[..] {
        let mut height_counts = HashMap::new();
        let (mut min, mut max) = (256, 0);

        (0..n).for_each(|_| {
            read_line(&mut buf);
            buf.split_whitespace().for_each(|s| {
                let height = parse_int(s);

                height_counts
                    .entry(height)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                min = height.min(min);
                max = height.max(max);
            });
        });

        let time_heights: Vec<_> = (min..=max)
            .filter_map(|height| {
                let mut blocks = b;
                let mut time = 0;

                for (&h, &c) in height_counts.iter() {
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

                (blocks >= 0).then(|| (time, height))
            })
            .collect();

        let (min_time, _) = time_heights.iter().min_by_key(|(t, _)| t).unwrap();
        let (time, height) = time_heights
            .iter()
            .filter(|(t, _)| t == min_time)
            .max_by_key(|(_, h)| h)
            .unwrap();

        println!("{time} {height}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(parse_int).collect()
}
