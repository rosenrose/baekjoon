use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (x1, y1, r1, x2, y2, r2) = (input(), input(), input(), input(), input(), input());

        if (x1, y1) == (x2, y2) {
            println!("{}", if r1 == r2 { -1 } else { 0 });
            continue;
        }

        let dist = get_distance((x1, y1), (x2, y2));
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

fn get_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    ((x1 - x2) as f64).hypot((y1 - y2) as f64)
}
