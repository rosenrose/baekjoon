fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r1, c1, r2, c2] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut max = 0;

    let vortex: Vec<Vec<_>> = (r1..=r2)
        .map(|y| {
            (c1..=c2)
                .map(|x| {
                    let num = x.abs().max(y.abs());
                    let diff = num * 2;

                    let right_bottom = (num * 2 + 1).pow(2);
                    let left_bottom = right_bottom - diff;
                    let left_top = left_bottom - diff;
                    let right_top = left_top - diff;

                    let cell = match (x, y) {
                        (x, y) if y == num => right_bottom - x.abs_diff(num) as i32,
                        (x, y) if x == -num => left_bottom - y.abs_diff(num) as i32,
                        (x, y) if y == -num => left_top - x.abs_diff(-num) as i32,
                        (x, y) if x == num => right_top - y.abs_diff(-num) as i32,
                        _ => unreachable!(),
                    };

                    max = cell.max(max);

                    cell
                })
                .collect()
        })
        .collect();

    let width = max.ilog10() as usize + 1;

    for row in vortex {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{cell:width$}");
        }
        println!("");
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
