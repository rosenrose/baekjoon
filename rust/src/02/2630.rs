use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let paper: Vec<String> = input.map(|row| row.replace(' ', "")).collect();
    let (mut white, mut blue) = (0, 0);

    cut(&paper, n, 0, 0, &mut white, &mut blue);

    println!("{white}\n{blue}");
}

fn cut(paper: &[String], n: usize, x: usize, y: usize, white: &mut i32, blue: &mut i32) {
    let (mut count_0, mut count_1) = (0, 0);

    for row in &paper[y..y + n] {
        for cell in row[x..x + n].chars() {
            match cell {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => unreachable!(),
            }
        }
    }

    if count_0 == n * n {
        *white += 1;
        return;
    }

    if count_1 == n * n {
        *blue += 1;
        return;
    }

    let half = n / 2;

    cut(paper, half, x, y, white, blue);
    cut(paper, half, x + half, y, white, blue);
    cut(paper, half, x, y + half, white, blue);
    cut(paper, half, x + half, y + half, white, blue);
}
