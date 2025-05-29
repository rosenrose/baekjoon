use std::io;

const WIDTH_MAX: usize = 5;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [r1, c1, r2, c2] = [(); 4].map(|_| input.next().unwrap());
    let (width, height) = ((c2 - c1 + 1) as usize, (r2 - r1 + 1) as usize);
    let mut max = 0;
    let mut vortex = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for r in r1..=r2 {
        for c in c1..=c2 {
            let num = c.abs().max(r.abs());
            let diff = num * 2;

            let right_bottom = (num * 2 + 1).pow(2);
            let left_bottom = right_bottom - diff;
            let left_top = left_bottom - diff;
            let right_top = left_top - diff;

            let cell = match (c, r) {
                (_, y) if y == num => right_bottom - c.abs_diff(num) as i32,
                (x, _) if x == -num => left_bottom - r.abs_diff(num) as i32,
                (_, y) if y == -num => left_top - c.abs_diff(-num) as i32,
                (x, _) if x == num => right_top - r.abs_diff(-num) as i32,
                _ => unreachable!(),
            };

            max = cell.max(max);
            vortex[(r - r1) as usize][(c - c1) as usize] = cell;
        }
    }

    let max_len = max.ilog10() as usize + 1;

    for row in &vortex[..height] {
        for (i, cell) in row[..width].iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{cell:max_len$}");
        }
        println!("");
    }
}
