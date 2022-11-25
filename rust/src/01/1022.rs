fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = parse_int_vec(&buf);
    let (r1, c1, r2, c2) = (nums[0], nums[1], nums[2], nums[3]);

    let mut max = 0;
    let vortex: Vec<Vec<_>> = (r1..=r2)
        .map(|y| {
            (c1..=c2)
                .map(|x| {
                    let num = x.abs().max(y.abs());
                    let diff = num * 2;
                    let (left_top_y, left_top_x) = (-num, -num);

                    let right_bottom = (num * 2 + 1).pow(2);
                    let left_bottom = right_bottom - diff;
                    let left_top = left_bottom - diff;
                    let right_top = left_top - diff;

                    let cell = match (x, y) {
                        (x, y) if y == -num => left_top - (x - left_top_x),
                        (x, y) if y == num => left_bottom + (x - left_top_x),
                        (x, y) if x == -num => left_top + (y - left_top_y),
                        (x, y) if x == num => right_top - (y - left_top_y),
                        _ => 0,
                    };

                    max = cell.max(max);

                    cell
                })
                .collect()
        })
        .collect();

    let width = max.to_string().len();

    for row in vortex {
        for (i, cell) in row.iter().enumerate() {
            print!("{cell:width$}");

            if i < row.len() - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
