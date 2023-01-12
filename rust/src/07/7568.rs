use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let mut volumes: Vec<_> = (0..n).map(|_| ((input(), input()), 1)).collect();

    for i in 0..n {
        for j in i + 1..n {
            let ((height1, weight1), _) = volumes[i];
            let ((height2, weight2), _) = volumes[j];

            if height1 > height2 && weight1 > weight2 {
                let (_, order2) = &mut volumes[j];
                *order2 += 1;
            }
            if height1 < height2 && weight1 < weight2 {
                let (_, order1) = &mut volumes[i];
                *order1 += 1;
            }
        }
    }

    for (_, rank) in volumes {
        print!("{rank} ");
    }
}
