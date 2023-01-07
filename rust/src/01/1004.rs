use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let (start, end) = ((input(), input()), (input(), input()));

        let n = input();
        let mut count = 0;

        for _ in 0..n {
            let planet = ((input(), input()), input() as f64);

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

fn distance_of_points((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    ((x1 - x2) as f64).hypot((y1 - y2) as f64)
}

fn is_point_inside_circle(point: (i32, i32), (center, radius): ((i32, i32), f64)) -> bool {
    distance_of_points(point, center) < radius
}
