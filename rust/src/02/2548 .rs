use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut nums = Vec::new();
    let mut nums_set = HashSet::new();

    for token in buf.split_whitespace() {
        let num: i32 = token.parse().unwrap();

        nums.push(num);
        nums_set.insert(num);
    }

    let mut min_diff_sum = i32::MAX;

    let diff_sums: Vec<(i32, i32)> = nums_set
        .iter()
        .map(|&num| {
            let diff_sum = nums.iter().map(|n| (n - num).abs()).sum::<i32>();
            if diff_sum < min_diff_sum {
                min_diff_sum = diff_sum;
            }

            (num, diff_sum)
        })
        .collect();

    let represents = diff_sums.iter().filter(|(_, diff)| *diff == min_diff_sum);

    let (represent, _) = represents.min_by_key(|(num, _)| num).unwrap();

    println!("{represent}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
