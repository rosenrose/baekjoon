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

    let mut min_diff_sum = u32::MAX;

    let diff_sums: Vec<(i32, u32)> = nums_set
        .iter()
        .map(|&num| {
            let diff_sum = nums.iter().map(|n| n.abs_diff(num)).sum::<u32>();
            min_diff_sum = diff_sum.min(min_diff_sum);

            (num, diff_sum)
        })
        .collect();

    let represents = diff_sums
        .iter()
        .filter(|(_, diff)| *diff == min_diff_sum)
        .map(|(num, _)| num);

    let represent = represents.min().unwrap();

    println!("{represent}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
