use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    const YES: &str = "YES";

    let (x1, y1, r1, x2, y2, r2) = (input(), input(), input(), input(), input(), input());

    let dist = get_distance((x1, y1), (x2, y2));
    let bigger = r1.max(r2) as f64;
    let is_inscribed = dist <= bigger;

    if is_inscribed {
        println!("{YES}");
        return;
    }

    println!("{}", if (r1 + r2) as f64 > dist { YES } else { "NO" });
}

fn get_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    ((x1 - x2) as f64).hypot((y1 - y2) as f64)
}
