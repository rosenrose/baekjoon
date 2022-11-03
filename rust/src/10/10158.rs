fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let size = parse_int_vec(&buf);
    read_line(&mut buf);

    let coord = parse_int_vec(&buf);
    read_line(&mut buf);

    let t: i32 = buf.trim().parse().unwrap();

    if let [w, h, p, q] = [size, coord].concat()[..] {
        let (mut ant_x, mut ant_y) = (p, q);

        let mut time = t;
        let horizontal_dist = time.min(w - ant_x);

        ant_x += horizontal_dist;
        time -= horizontal_dist;
        time %= w * 2;

        ant_x = if time < w { ant_x - time } else { time % w };

        time = t;
        let vertical_dist = time.min(h - ant_y);

        ant_y += vertical_dist;
        time -= vertical_dist;
        time %= h * 2;

        ant_y = if time < h { ant_y - time } else { time % h };

        println!("{ant_x} {ant_y}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
