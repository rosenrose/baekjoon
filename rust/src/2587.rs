fn main() {
    let mut buf = String::new();

    let mut nums: Vec<usize> = (0..5)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .collect();

    nums.sort();

    println!("{}\n{}", nums.iter().sum::<usize>() / nums.len(), nums[2]);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> usize {
    buf.trim().parse().unwrap()
}
