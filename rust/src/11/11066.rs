use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut memo = vec![vec![(0, 0); 500]; 500];

    for _ in 0..input() {
        let n = input() as usize;

        for i in 0..n {
            let (size, cost) = (input(), 0);
            memo[i][i] = (size, cost);
        }

        for window_size in 2..=n {
            let (mut i, mut j) = (0, window_size - 1);

            while j < n {
                memo[i][j].0 = memo[i][i].0 + memo[i + 1][j].0;
                memo[i][j].1 = (i..j)
                    .map(|k| (memo[i][k].0 + memo[i][k].1) + (memo[k + 1][j].0 + memo[k + 1][j].1))
                    .min()
                    .unwrap();

                i += 1;
                j += 1;
            }
        }
        // for r in &memo[..n] {
        //     println!("{:?}", &r[..n]);
        // }
        println!("{}", memo[0][n - 1].1);
    }
}
