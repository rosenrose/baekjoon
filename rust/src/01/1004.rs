use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (start, end) = (
            (input.next().unwrap(), input.next().unwrap()),
            (input.next().unwrap(), input.next().unwrap()),
        );

        let n = input.next().unwrap();
        let mut count = 0;

        for _ in 0..n {
            let planet = (
                (input.next().unwrap(), input.next().unwrap()),
                input.next().unwrap() as f64,
            );

            let is_start_inside_planet = is_point_inside_circle(start, planet);
            let is_end_inside_planet = is_point_inside_circle(end, planet);

            if is_start_inside_planet {
                count += 1;
            }
            if is_end_inside_planet {
                count += 1;
            }

            if is_start_inside_planet && is_end_inside_planet {
                count -= 2;
            }
        }

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn distance_of_points(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt()
}

fn is_point_inside_circle(point: (i32, i32), circle: ((i32, i32), f64)) -> bool {
    let (center, radius) = circle;

    distance_of_points(point, center) < radius
}
