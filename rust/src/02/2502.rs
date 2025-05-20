use std::io;

const MAX: usize = 30 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [d, k] = [(); 2].map(|_| input.next().unwrap());

    let mut memo = [0; MAX];
    memo[d] = k as i32;

    'outer: for prev in (k + 1) / 2..k {
        memo[d - 1] = prev as i32;

        for i in (1..=d - 2).rev() {
            memo[i] = memo[i + 2] - memo[i + 1];

            if memo[i] < 1 || memo[i] > memo[i + 1] {
                continue 'outer;
            }
        }

        break;
    }
    // println!("{memo:?}");
    println!("{}\n{}", memo[1], memo[2]);
}
