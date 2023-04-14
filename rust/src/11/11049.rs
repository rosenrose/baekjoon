use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input() as usize;
    let mut sizes = vec![0; n + 1];
    (sizes[0], sizes[1]) = (input(), input());

    for i in 2..=n {
        input();
        sizes[i] = input();
    }

    let mut memo = vec![vec![0; n + 1]; n + 1];

    for start in 2..=n {
        let (mut i, mut j) = (1, start);

        for _ in 0..n - start + 1 {
            memo[i][j] = (i..j)
                .map(|k| memo[i][k] + memo[k + 1][j] + (sizes[i - 1] * sizes[k] * sizes[j]))
                .min()
                .unwrap();
            i += 1;
            j += 1;
        }
    }
    // println!("{memo:?}");
    println!("{}", memo[1][n]);
}
