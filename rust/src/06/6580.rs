use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let (width, _height, _) = (input(), input(), input());
    let n: usize = width.split(' ').next_back().unwrap().parse().unwrap();
    const HEX_PREFIX: &str = "0x";

    let map: Vec<Vec<_>> = (0..n)
        .map(|_| {
            input()
                .split_terminator(',')
                .flat_map(|hex| {
                    let mut hex = u8::from_str_radix(&hex[HEX_PREFIX.len()..], 16).unwrap();

                    (0..8).map(move |_| {
                        let ch = if hex & 1 == 0 { 'W' } else { 'B' };
                        hex >>= 1;
                        ch
                    })
                })
                .collect()
        })
        .collect();
    // println!("{map:?}");
    println!("{n}\n{}", compress(&map, 0, 0, n));
}

fn compress(map: &Vec<Vec<char>>, x: usize, y: usize, n: usize) -> String {
    if n == 1 {
        return map[y][x].to_string();
    }

    let (mut count_w, mut count_b) = (0, 0);

    for row in map[y..y + n].iter() {
        for cell in row[x..x + n].iter() {
            match cell {
                'W' => count_w += 1,
                'B' => count_b += 1,
                _ => (),
            }
        }
    }

    if count_w == n * n {
        return 'W'.to_string();
    }
    if count_b == n * n {
        return 'B'.to_string();
    }

    format!(
        "Q{}{}{}{}",
        compress(map, x, y, n / 2),
        compress(map, x + n / 2, y, n / 2),
        compress(map, x, y + n / 2, n / 2),
        compress(map, x + n / 2, y + n / 2, n / 2)
    )
}
