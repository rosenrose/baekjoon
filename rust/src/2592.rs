fn main() {
    let mut buf = String::new();

    let nums: Vec<usize> = (0..10)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .collect();

    let avg = nums.iter().sum::<usize>() / nums.len();

    let counts: Vec<(usize, usize)> = nums
        .iter()
        .map(|&num| {
            let count = nums.iter().filter(|&n| *n == num).count();
            (num, count)
        })
        .collect();

    let (mode, _) = counts.iter().max_by_key(|(_, c)| c).unwrap();

    println!("{avg}\n{mode}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> usize {
    buf.trim().parse().unwrap()
}
