fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let counts = nums.iter().map(|&num| {
        let count = nums.iter().filter(|&n| *n == num).count();
        (num, count)
    });

    let mut prize = 0;

    if counts.clone().all(|(_, c)| c == 3) {
        prize = 10000 + nums[0] * 1000;
    } else if counts.clone().all(|(_, c)| c == 1) {
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
