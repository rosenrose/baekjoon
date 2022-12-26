use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (x1, y1, r1, x2, y2, r2) = (input(), input(), input(), input(), input(), input());

        if (x1, y1) == (x2, y2) {
            println!("{}", if r1 == r2 { -1 } else { 0 });
            continue;
        }

        let dist = distance_of_points((x1, y1), (x2, y2));
        let (bigger, smaller) = (r1.max(r2) as f64, r1.min(r2) as f64);

        let is_inscribed = dist < bigger;

        let joints = if is_inscribed {
            if (smaller - (bigger - dist)).abs() < 1e-10 {
                1
            } else {
                if smaller < bigger - dist {
                    0
                } else {
                    2
                }
            }
        } else {
            let radius_sum = r1 as f64 + r2 as f64;

            if (radius_sum - dist).abs() < 1e-10 {
                1
            } else {
                if radius_sum < dist {
                    0
                } else {
                    2
                }
            }
        };

        println!("{joints}");
    }
}

fn distance_of_points(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt()
}
