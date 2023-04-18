use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (c, r, k) = (input(), input(), input());
    let (row, col) = (c, r);

    if k > c * r {
        println!("0");
        return;
    }

    let mut seats = vec![vec![0; col as usize]; row as usize];
    let mut num = 0;
    let (mut x, mut y) = (-1, 0);
    let mut direction = (1, 0);

    while num <= k {
        loop {
            let (next_x, next_y) = (x + direction.0, y + direction.1);

            if (next_x == -1 || next_x == col || next_y == -1 || next_y == row)
                || seats[next_y as usize][next_x as usize] != 0
            {
                break;
            }

            (x, y) = (next_x, next_y);
            num += 1;
            seats[y as usize][x as usize] = num;

            if num == k {
                // println!("{seats:?}");
                println!("{} {}", y + 1, x + 1);
                return;
            }
        }

        direction = match direction {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => Default::default(),
        };
    }
}