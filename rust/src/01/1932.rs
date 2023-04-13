use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut max_sum = Vec::new();

    for i in 1..=n {
        if i == 1 {
            max_sum.push(input());
            continue;
        }

        let mut next = vec![0; i];

        for (j, num) in (0..i).map(|j| (j, input())) {
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
