use std::io;

const MAX: usize = 500;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut max_sum = [0; MAX];

    for i in 1..=n {
        if i == 1 {
            max_sum[0] = input.next().unwrap();
            continue;
        }

        let mut next = [0; MAX];

        for (j, num) in input.by_ref().take(i).enumerate() {
            next[j] = num
                + match j {
                    0 => max_sum[0],
                    j if j == i - 1 => max_sum[j - 1],
                    _ => max_sum[j - 1].max(max_sum[j]),
                };
        }

        max_sum = next;
    }
    // println!("{max_sum:?}");
    println!("{}", max_sum.iter().max().unwrap());
}
