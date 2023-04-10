use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    const M: i32 = 1_000_000_009;
    let mut memo = vec![0; 1_000_001];
    (memo[1], memo[2], memo[3]) = (1, 2, 4);

    for i in 4..=1_000_000 {
        memo[i] = ((memo[i - 3] + memo[i - 2]) % M + memo[i - 1]) % M;
    }

    for num in input.skip(1) {
        writeln!(output, "{}", memo[num]).unwrap();
    }

    print!("{output}");
}
