fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: usize = buf.trim().parse().unwrap();
    let paper = parse_str_vec_lines(&mut buf, n);
    let (mut white, mut blue) = (0, 0);

    cut(&paper, 0, 0, n, &mut white, &mut blue);

    println!("{white}\n{blue}");
}

fn cut(paper: &Vec<String>, x: usize, y: usize, n: usize, white: &mut i32, blue: &mut i32) {
    let (mut count_0, mut count_1) = (0, 0);

    paper.iter().skip(y).take(n).for_each(|row| {
        row.chars().skip(x).take(n).for_each(|cell| match cell {
            '0' => count_0 += 1,
            '1' => count_1 += 1,
            _ => (),
        })
    });

    if count_0 == n * n {
        *white += 1;

        return;
    }

    if count_1 == n * n {
        *blue += 1;

        return;
    }

    cut(paper, x, y, n / 2, white, blue);
    cut(paper, x + n / 2, y, n / 2, white, blue);
    cut(paper, x, y + n / 2, n / 2, white, blue);
    cut(paper, x + n / 2, y + n / 2, n / 2, white, blue);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.split_whitespace().collect()
        })
        .collect()
}
