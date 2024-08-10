use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input() as usize;
    let sizes: Vec<_> = (0..n).map(|_| (input(), input())).collect();
    let mut memo = [[0; 500]; 500];

    for window_size in 2..=n {
        let (mut i, mut j) = (0, window_size - 1);

        while j < n {
            memo[i][j] = (i..j)
                .map(|k| memo[i][k] + memo[k + 1][j] + (sizes[i].0 * sizes[k].1 * sizes[j].1))
                .min()
                .unwrap();
            i += 1;
            j += 1;
        }
    }
    // println!("{memo:?}");
    println!("{}", memo[0][n - 1]);
}
