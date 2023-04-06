use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let paper: Vec<String> = input.map(|row| row.split(' ').collect()).collect();
    let (mut white, mut blue) = (0, 0);

    cut(&paper, 0, 0, n, &mut white, &mut blue);

    println!("{white}\n{blue}");
}

fn cut(paper: &Vec<String>, x: usize, y: usize, n: usize, white: &mut i32, blue: &mut i32) {
    let (mut count_0, mut count_1) = (0, 0);

    for row in paper[y..y + n].iter() {
        for cell in row[x..x + n].chars() {
            match cell {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => (),
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

    cut(paper, x, y, n / 2, white, blue);
    cut(paper, x + n / 2, y, n / 2, white, blue);
    cut(paper, x, y + n / 2, n / 2, white, blue);
    cut(paper, x + n / 2, y + n / 2, n / 2, white, blue);
}
