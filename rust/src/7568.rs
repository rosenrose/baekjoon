fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: usize = buf.trim().parse().unwrap();

    let mut volumes: Vec<((i32, i32), usize)> = (0..n)
        .map(|_| {
            read_line(&mut buf);
            let nums = parse_int_vec(&buf);

            ((nums[0], nums[1]), 1)
        })
        .collect();

    for i in 0..n {
        for j in i + 1..n {
            let ((height1, weight1), _) = volumes[i];
            let ((height2, weight2), _) = volumes[j];

            if height1 > height2 && weight1 > weight2 {
                volumes[j].1 += 1;
            }
            if height1 < height2 && weight1 < weight2 {
                volumes[i].1 += 1;
            }
        }
    }

    for (_, rank) in volumes {
        print!("{rank} ");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
