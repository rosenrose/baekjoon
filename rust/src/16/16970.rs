fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m, k] = parse_int_vec(&buf)[..] else { return };
    let points: Vec<_> = (0..=n).flat_map(|x| (0..=m).map(move |y| (x, y))).collect();
    let mut count = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let ((x1, y1), (x2, y2)) = (points[i], points[j]);
            let (width, height) = (x1.abs_diff(x2), y1.abs_diff(y2));

            if get_gcd(width, height) + 1 != k {
                continue;
            }

            count += 1;
        }
    }

    println!("{count}");
}

fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &String) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
