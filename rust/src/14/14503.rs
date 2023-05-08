use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, mut r, mut c, mut d) = (input(), input(), input(), input(), input());

    let mut room: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| input()).collect()).collect();

    loop {
        room[r][c] = 2;

        let (north, east, south, west) = (
            room[r - 1][c],
            room[r][c + 1],
            room[r + 1][c],
            room[r][c - 1],
        );

        let (up, right, down, left) = match d {
            0 => (north, east, south, west),
            1 => (east, south, west, north),
            2 => (south, west, north, east),
            3 => (west, north, east, south),
            _ => (0, 0, 0, 0),
        };

        if [left, right, up, down].iter().all(|&s| s == 1 || s == 2) {
            let back;

            (back, r, c) = match d {
                0 => (south, r + 1, c),
                1 => (west, r, c - 1),
                2 => (north, r - 1, c),
                3 => (east, r, c + 1),
                _ => (0, 0, 0),
            };

            if back == 1 {
                break;
            }

            continue;
        }

        if left == 0 {
            match d {
                0 => c -= 1,
                1 => r -= 1,
                2 => c += 1,
                3 => r += 1,
                _ => unreachable!(),
            };
        }

        d = (d + 3) % 4;
    }

    println!(
        "{}",
        room.iter()
            .map(|row| row.iter().filter(|&&cell| cell == 2).count())
            .sum::<usize>()
    );

    // for row in room {
    //     println!("{row:?}");
    // }
}
