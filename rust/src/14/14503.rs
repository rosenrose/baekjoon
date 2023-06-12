use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, m, mut r, mut c, mut d] = [(); 5].map(|_| input());
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
            _ => unreachable!(),
        };

        if [left, right, up, down]
            .iter()
            .all(|&dir| dir == 1 || dir == 2)
        {
            let back;

            (back, r, c) = match d {
                0 => (south, r + 1, c),
                1 => (west, r, c - 1),
                2 => (north, r - 1, c),
                3 => (east, r, c + 1),
                _ => unreachable!(),
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
        room.iter().flatten().filter(|&&cell| cell == 2).count()
    );
}
