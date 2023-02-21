use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut volume_ranks: Vec<_> = (0..n).map(|_| ((input(), input()), 1)).collect();

    for i in 0..n - 1 {
        let ((height1, weight1), _) = volume_ranks[i];

        for j in i + 1..n {
            let ((height2, weight2), _) = volume_ranks[j];

            if height1 > height2 && weight1 > weight2 {
                volume_ranks[j].1 += 1;
            }
            if height1 < height2 && weight1 < weight2 {
                volume_ranks[i].1 += 1;
            }
        }
    }

    for (_, rank) in volume_ranks {
        print!("{rank} ");
    }
}
