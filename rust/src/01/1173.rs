fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, min, max, inc, dec] = parse_int_vec(&buf)[..] else { return };

    if inc > (max - min) {
        println!("-1");
        return;
    }

    let mut current = min;
    let (mut exercise, mut time) = (0, 0);

    'outer: loop {
        while (max - current) >= inc {
            current += inc;
            exercise += 1;
            time += 1;

            if exercise >= n {
                break 'outer;
            }
        }

        while (max - current) < inc {
            current = min.max(current - dec);
            time += 1;
        }
    }

    println!("{time}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
