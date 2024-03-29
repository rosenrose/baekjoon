use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let [w, h, x, y, p] = [(); 5].map(|_| input());
    let radius = h / 2;
    let mut count = 0;

    for player in (0..p).map(|_| (input(), input())) {
        if is_point_inside_rect(player, (x, y, w, h))
            || is_point_inside_circle(player, ((x, y + radius), radius as f64))
            || is_point_inside_circle(player, ((x + w, y + radius), radius as f64))
        {
            count += 1;
        }
    }

    println!("{count}");
}

fn is_point_inside_rect(
    (px, py): (i32, i32),
    (rx, ry, width, height): (i32, i32, i32, i32),
) -> bool {
    rx <= px && px <= rx + width && ry <= py && py <= ry + height
}

fn is_point_inside_circle(point: (i32, i32), (center, radius): ((i32, i32), f64)) -> bool {
    get_distance(point, center) <= radius
}

fn get_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    ((x1 - x2) as f64).hypot((y1 - y2) as f64)
}
