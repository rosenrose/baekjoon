use std::io;

const MAX: usize = 64;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut video = [""; MAX];

    for (r, row) in input.enumerate() {
        video[r] = row;
    }

    println!("{}", compress(&video[..n], n, 0, 0));
}

fn compress(video: &[&str], n: usize, x: usize, y: usize) -> String {
    if n == 1 {
        return video[y].chars().nth(x).unwrap().to_string();
    }

    let (mut count_0, mut count_1) = (0, 0);

    for row in &video[y..y + n] {
        for cell in row[x..x + n].chars() {
            match cell {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => unreachable!(),
            }
        }
    }

    if count_0 == n * n {
        return '0'.to_string();
    }
    if count_1 == n * n {
        return '1'.to_string();
    }

    let half = n / 2;

    format!(
        "({}{}{}{})",
        compress(video, half, x, y),
        compress(video, half, x + half, y),
        compress(video, half, x, y + half),
        compress(video, half, x + half, y + half)
    )
}
