use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut sums = Vec::<i32>::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as usize;
        sums.reserve(n * (n + 1) / 2);

        let mut max_sum = i32::MIN;

        for (i, num) in input.by_ref().take(n).enumerate() {
            for j in sums.len() - i..sums.len() {
                let sum = sums[j] + num;

                max_sum = sum.max(max_sum);
                sums.push(sum);
            }

            max_sum = num.max(max_sum);
            sums.push(num);
        }
        sums.clear();

        println!("{max_sum}");
    }
}
