use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, find_num] = [(); 2].map(|_| input.next().unwrap());
    let (mut i, mut j) = (0, 0);

    for y in -n / 2..=n / 2 {
        for x in -n / 2..=n / 2 {
            let num = x.abs().max(y.abs());
            let diff = num * 2;

            let left_top = (num * 2 + 1).pow(2);
            let left_bottom = left_top - diff;
            let right_bottom = left_bottom - diff;
            let right_top = right_bottom - diff;

            let cell = match (x, y) {
                (x, y) if x == -num => left_top - y.abs_diff(-num) as i32,
                (x, y) if y == num => left_bottom - x.abs_diff(-num) as i32,
                (x, y) if x == num => right_bottom - y.abs_diff(num) as i32,
                (x, y) if y == -num => right_top - x.abs_diff(num) as i32,
                _ => unreachable!(),
            };

            if cell == find_num {
                (i, j) = ((y + n / 2) + 1, (x + n / 2) + 1);
            }

            write!(output, "{cell} ").unwrap();
        }
        writeln!(output, "").unwrap();
    }

    writeln!(output, "{i} {j}").unwrap();
    print!("{output}");
}
