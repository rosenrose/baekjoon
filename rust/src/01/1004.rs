fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);

        if let [x1, y1, x2, y2] = parse_int_vec(&buf)[..] {
            let (start, end) = ((x1, y1), (x2, y2));

            read_line(&mut buf);
            let planets = parse_int(&buf);

            let mut count = 0;

            for _ in 0..planets {
                read_line(&mut buf);

                if let [cx, cy, r] = parse_int_vec(&buf)[..] {
                    let planet = ((cx, cy), r as f64);

                    let is_start_inside_planet = is_point_inside_circle(start, planet);
                    let is_end_inside_plaent = is_point_inside_circle(end, planet);

                    if is_start_inside_planet {
                        count += 1;
                    }
                    if is_end_inside_plaent {
                        count += 1;
                    }

                    if is_start_inside_planet && is_end_inside_plaent {
                        count -= 2;
                    }
                }
            }

            println!("{count}");
        }
    }
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
