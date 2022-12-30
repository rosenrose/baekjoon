use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (w, h, x, y, p) = (input(), input(), input(), input(), input());
    let radius = h / 2;
    let mut count = 0;

    for _ in 0..p {
        let player = (input(), input());

        if is_point_inside_or_border_rect(player, (x, y, w, h))
            || is_point_inside_or_border_circle(player, ((x, y + radius), radius as f64))
            || is_point_inside_or_border_circle(player, ((x + w, y + radius), radius as f64))
        {
            count += 1;
        }
    }

    println!("{count}");
}

fn is_point_inside_or_border_rect(
    (px, py): (i32, i32),
    (rx, ry, width, height): (i32, i32, i32, i32),
) -> bool {
    (rx..=rx + width).contains(&px) && (ry..=ry + height).contains(&py)
}

fn is_point_inside_or_border_circle(
    point: (i32, i32),
    (center, radius): ((i32, i32), f64),
) -> bool {
    distance_of_points(point, center) <= radius
}

fn distance_of_points((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    ((x1 - x2) as f64).hypot((y1 - y2) as f64)
}
