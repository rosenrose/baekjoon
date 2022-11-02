fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        if let [x1, y1, r1, x2, y2, r2] = parse_int_vec(&buf)[..] {
            if (x1, y1) == (x2, y2) {
                println!("{}", if r1 == r2 { -1 } else { 0 });
                continue;
            }

            let joints;

            let dist = distance_of_points((x1, y1), (x2, y2));
            let (bigger, smaller) = (r1.max(r2) as f64, r1.min(r2) as f64);

            let is_inscribed = dist < bigger;

            if is_inscribed {
                if (smaller - (bigger - dist)).abs() < 1e-10 {
                    joints = 1;
                } else {
                    joints = if smaller < bigger - dist { 0 } else { 2 };
                }
            } else {
                let radius_sum = r1 as f64 + r2 as f64;

                if (radius_sum - dist).abs() < 1e-10 {
                    joints = 1;
                } else {
                    joints = if radius_sum < dist { 0 } else { 2 };
                }
            }

            println!("{joints}");
        }
    }
}

fn distance_of_points(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt()
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
