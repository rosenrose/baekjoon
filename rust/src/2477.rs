struct Side {
    direction: i32,
    length: i32,
}

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let k: i32 = buf.trim().parse().unwrap();

    let mut sides: Vec<_> = (0..6)
        .map(|_| {
            read_line(&mut buf);
            let nums = parse_int_vec(&buf);

            Side {
                direction: nums[0],
                length: nums[1],
            }
        })
        .collect();

    let directions: Vec<i32> = sides.iter().map(|side| side.direction).collect();
    let mut outers: Vec<&i32> = directions
        .iter()
        .filter(|&direction| directions.iter().filter(|&d| d == direction).count() == 1)
        .collect();

    outers.sort();

    let (init_direction, outer_width_idx, outer_height_idx, inner_width_idx, inner_height_idx) =
        match (outers[0], outers[1]) {
            (1, 3) => (1, 0, 5, 2, 3),
            (1, 4) => (1, 0, 1, 4, 3),
            (2, 3) => (2, 0, 1, 4, 3),
            (2, 4) => (2, 0, 5, 2, 3),
            _ => (0, 0, 0, 0, 0),
        };

    while sides[0].direction != init_direction {
        let temp = sides.pop().unwrap();
        sides.insert(0, temp);
    }

    let outer_area = sides[outer_width_idx].length * sides[outer_height_idx].length;
    let inner_area = sides[inner_width_idx].length * sides[inner_height_idx].length;
    let count = (outer_area - inner_area) * k;

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/*
┌┐               ┌┐             ┌─┐             ┌─┐
│└┐             ┌┘│             │┌┘             └┐│
└─┘             └─┘             └┘               └┘
1 4 2 4 2 3  |  1 4 2 3 2 3  |  2 3 1 4 1 4  |  2 3 1 3 1 4
(1, 3)          (1, 4)          (2, 3)          (2, 4)
*/
