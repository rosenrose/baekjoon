use std::io::{stdin, Read};

struct Side {
    direction: i32,
    length: i32,
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let k = input.next().unwrap();
    let mut direction_counts = [0; 5];

    let mut sides: Vec<_> = (0..6)
        .map(|_| {
            let (direction, length) = (input.next().unwrap(), input.next().unwrap());

            direction_counts[direction as usize] += 1;

            Side { direction, length }
        })
        .collect();

    let mut outers: Vec<_> = direction_counts
        .iter()
        .enumerate()
        .filter_map(|(d, &c)| (c == 1).then(|| d))
        .collect();

    outers.sort();

    let (init_direction, out_width_idx, out_height_idx, in_width_idx, in_height_idx) =
        match outers[..] {
            [1, 3] => (1, 0, 5, 2, 3),
            [1, 4] => (1, 0, 1, 4, 3),
            [2, 3] => (2, 0, 1, 4, 3),
            [2, 4] => (2, 0, 5, 2, 3),
            _ => (0, 0, 0, 0, 0),
        };

    while sides[0].direction != init_direction {
        let temp = sides.pop().unwrap();
        sides.insert(0, temp);
    }

    let outer_area = sides[out_width_idx].length * sides[out_height_idx].length;
    let inner_area = sides[in_width_idx].length * sides[in_height_idx].length;
    let count = (outer_area - inner_area) * k;

    println!("{count}");
}

/*
┌┐               ┌┐             ┌─┐             ┌─┐
│└┐             ┌┘│             │┌┘             └┐│
└─┘             └─┘             └┘               └┘
1 4 2 4 2 3  |  1 4 2 3 2 3  |  2 3 1 4 1 4  |  2 3 1 3 1 4
(1, 3)          (1, 4)          (2, 3)          (2, 4)
*/
