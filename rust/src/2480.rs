fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = parse_int_vec(&buf);

    let counts: Vec<(i32, usize)> = nums
        .iter()
        .map(|&num| {
            let count = nums.iter().filter(|&n| *n == num).count();
            (num, count)
        })
        .collect();

    let mut prize = 0;

    if counts.iter().all(|&(_, c)| c == 3) {
        prize = 10000 + nums[0] * 1000;
    } else if counts.iter().all(|&(_, c)| c == 1) {
        prize = nums.iter().max().unwrap() * 100;
    } else {
        for (num, count) in counts {
            if count == 2 {
                prize = 1000 + num * 100;
                break;
            }
        }
    }

    println!("{prize}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
