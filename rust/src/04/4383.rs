use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    const JOLLY: &str = "Jolly";

    while let Some(n) = input.next() {
        let n = n as usize;
        let nums: Vec<_> = input.by_ref().take(n).collect();

        if n == 1 {
            println!("{JOLLY}");
            continue;
        }

        let mut jumps = vec![false; n];

        for i in 1..n {
            let diff = nums[i - 1].abs_diff(nums[i]) as usize;

            if diff < n {
                jumps[diff] = true;
            }
        }

        println!(
            "{}",
            if jumps[1..].iter().all(|&b| b) {
                JOLLY
            } else {
                "Not jolly"
            }
        );
    }
}
