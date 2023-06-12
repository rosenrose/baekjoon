use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [width, _height, _] = [(); 3].map(|_| input.next().unwrap());
    let n: usize = width.split(' ').next_back().unwrap().parse().unwrap();

    const HEX_PREFIX: &str = "0x";
    let map: Vec<Vec<_>> = input
        .take(n)
        .map(|line| {
            line.split_terminator(',')
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
    println!("{n}\n{}", compress(&map, n, 0, 0));
}

fn compress(map: &[Vec<char>], n: usize, x: usize, y: usize) -> String {
    if n == 1 {
        return map[y][x].to_string();
    }

    let (mut count_w, mut count_b) = (0, 0);

    for row in &map[y..y + n] {
        for cell in &row[x..x + n] {
            match cell {
                'W' => count_w += 1,
                'B' => count_b += 1,
                _ => unreachable!(),
            }
        }
    }

    if count_w == n * n {
        return 'W'.to_string();
    }
    if count_b == n * n {
        return 'B'.to_string();
    }

    let half = n >> 1;

    format!(
        "Q{}{}{}{}",
        compress(map, half, x, y),
        compress(map, half, x + half, y),
        compress(map, half, x, y + half),
        compress(map, half, x + half, y + half)
    )
}
