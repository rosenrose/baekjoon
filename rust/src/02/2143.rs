use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let t = input.next().unwrap();
    let n = input.next().unwrap() as usize;
    let mut a_sums = Vec::with_capacity(n * (n + 1) / 2);

    for (i, num) in input.by_ref().take(n).enumerate() {
        for j in a_sums.len() - i..a_sums.len() {
            a_sums.push(a_sums[j] + num);
        }
        a_sums.push(num);
    }

    let m = input.next().unwrap() as usize;
    let mut b_sums = Vec::with_capacity(m * (m + 1) / 2);

    for (i, num) in input.enumerate() {
        for j in b_sums.len() - i..b_sums.len() {
            b_sums.push(b_sums[j] + num);
        }
        b_sums.push(num);
    }

    a_sums.sort_unstable();
    b_sums.sort_unstable();
    // println!("{a_sums:?} {b_sums:?}");
    let count: usize = a_sums
        .iter()
        .map(|a| {
            let start = b_sums.partition_point(|b| a + b < t);
            let len = b_sums[..start].partition_point(|b| a + b == t);

            len
        })
        .sum();

    println!("{count}");
}
