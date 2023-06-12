use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [w, h, p, q, t] = [(); 5].map(|_| input.next().unwrap());
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
