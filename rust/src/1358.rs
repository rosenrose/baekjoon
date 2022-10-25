fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [w, h, x, y, p] = parse_int_vec(&buf)[..] {
        let radius = h / 2;
        let mut count = 0;

        for _ in 0..p {
            read_line(&mut buf);

            if let [px, py] = parse_int_vec(&buf)[..] {
                let player = (px, py);

                if is_point_inside_or_border_rect(player, (x, y, w, h))
                    || is_point_inside_or_border_circle(player, ((x, y + radius), radius as f64))
                    || is_point_inside_or_border_circle(
                        player,
                        ((x + w, y + radius), radius as f64),
                    )
                {
                    count += 1;
                }
            }
        }

        println!("{count}");
    }
}

fn is_point_inside_or_border_rect(point: (i32, i32), rect: (i32, i32, i32, i32)) -> bool {
    let (px, py) = point;
    let (rx, ry, width, height) = rect;

    (rx..=rx + width).contains(&px) && (ry..=ry + height).contains(&py)
}

fn is_point_inside_or_border_circle(point: (i32, i32), circle: ((i32, i32), f64)) -> bool {
    let (center, radius) = circle;

    distance_of_points(point, center) <= radius
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
