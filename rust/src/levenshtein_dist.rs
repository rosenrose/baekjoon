use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(str::as_bytes);

    let (source, target) = (input.next().unwrap(), input.next().unwrap());
    let mut memo = vec![vec![0; source.len() + 1]; target.len() + 1];

    for i in 0..=target.len() {
        for j in 0..=source.len() {
            memo[i][j] = match (i, j) {
                (0, _) => j,
                (_, 0) => i,
                _ => {
                    let change_cost = if source[j - 1] == target[i - 1] { 0 } else { 1 };

                    (memo[i - 1][j] + 1)
                        .min(memo[i][j - 1] + 1)
                        .min(memo[i - 1][j - 1] + change_cost)
                }
            }
        }
    }

    println!("{memo:?}");
}
