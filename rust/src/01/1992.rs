fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: usize = buf.trim().parse().unwrap();
    let video = parse_str_vec_lines(&mut buf, n);

    println!("{}", compress(&video, 0, 0, n));
}

fn compress(video: &Vec<String>, x: usize, y: usize, n: usize) -> String {
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
