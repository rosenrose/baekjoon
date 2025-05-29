use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut matrix = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            matrix[r][c] = num;
        }
    }

    let max_size = height.min(width);

    for size in (1..=max_size).rev() {
        for y in 0..=height - size {
            for x in 0..=width - size {
                let top = matrix[y];
                let bottom = matrix[y + size - 1];

                let (top_left, top_right) = (top[x], top[x + size - 1]);
                let (bottom_left, bottom_right) = (bottom[x], bottom[x + size - 1]);

                if top_left == top_right && top_right == bottom_left && bottom_left == bottom_right
                {
                    println!("{}", size * size);
                    return;
                }
            }
        }
    }
}
