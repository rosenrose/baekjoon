use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const M: i64 = 1_000_000_007;
    let mut cache = [0; 3334];
    (cache[0], cache[1], cache[2]) = (1, 3, 13);

    for i in 3..=3333 {
        cache[i] = ((5 * cache[i - 1]) % M - (3 * cache[i - 2]) % M + cache[i - 3] + M) % M;
    }
    // println!("{cache:?}");
    for n in input.skip(1) {
        if n % 3 != 0 {
            println!("0");
            continue;
        }

        println!("{}", cache[n / 3]);
    }
}
