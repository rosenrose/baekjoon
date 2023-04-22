use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as usize;
        let mut memo = vec![0; n + 1];
        let mut max_sum = i32::MIN;

        for (i, num) in input.by_ref().take(n).enumerate() {
            memo[i + 1] = num.max(memo[i] + num);
            max_sum = memo[i + 1].max(max_sum);
        }

        println!("{max_sum}");
    }
}
