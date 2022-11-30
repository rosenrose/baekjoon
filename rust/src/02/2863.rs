fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let ab = parse_int_vec(&buf);
    read_line(&mut buf);

    let cd = parse_int_vec(&buf);

    if let [a, b, c, d] = [ab, cd].concat()[..] {
        let mut rotate_sums = Vec::new();

        while rotate_sums.len() < 4 {
            let sum = (
                a * d + b * c,
                match rotate_sums.len() {
                    0 => c * d,
                    1 => b * d,
                    2 => a * b,
                    3 => a * c,
                    _ => 0,
                },
            );

            rotate_sums.push((rotate_sums.len(), sum));
        }

        let (min_rotate, _) = rotate_sums
            .iter()
            .max_by(|(_, a), (_, b)| (a.0 * b.1).cmp(&(b.0 * a.1)))
            .unwrap();

        println!("{min_rotate}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
