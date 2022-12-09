use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let video: Vec<_> = input.collect();

    println!("{}", compress(&video, 0, 0, n));
}

fn compress(video: &Vec<&str>, x: usize, y: usize, n: usize) -> String {
    if n == 1 {
        return video[y].chars().nth(x).unwrap().to_string();
    }

    let (mut count_0, mut count_1) = (0, 0);

    video.iter().skip(y).take(n).for_each(|row| {
        row.chars().skip(x).take(n).for_each(|cell| match cell {
            '0' => count_0 += 1,
            '1' => count_1 += 1,
            _ => (),
        })
    });

    if count_0 == n * n {
        return '0'.to_string();
    }
    if count_1 == n * n {
        return '1'.to_string();
    }

    format!(
        "({}{}{}{})",
        compress(video, x, y, n / 2),
        compress(video, x + n / 2, y, n / 2),
        compress(video, x, y + n / 2, n / 2),
        compress(video, x + n / 2, y + n / 2, n / 2)
    )
}
